use actix_files::{Files, NamedFile};
use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder, Result,
};
use log::info;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use url::Url;

use std::collections::HashMap;
use std::path::PathBuf;

use common::{
    error::Error,
    types::{self, ErrorResponse},
};

use derive_more::Display;

#[derive(Clone, Debug, Display)]
pub enum UserError {
    #[display(fmt = "unused")]
    InvalidUrl,
    #[display(fmt = "unused")]
    NotFound,
    #[display(fmt = "unused")]
    InternalError,
    #[display(fmt = "unused")]
    Other(String),
}

impl From<UserError> for Error {
    fn from(e: UserError) -> Self {
        match e {
            UserError::InvalidUrl => Error::InvalidUrl,
            UserError::NotFound => Error::NotFound,
            UserError::Other(s) => Error::Other(s),
            _ => Error::Other(format!("{e:?}")),
        }
    }
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InvalidUrl => StatusCode::BAD_REQUEST,
            UserError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body({
                let response = ErrorResponse::new(self.clone().into());

                serde_json::to_string(&response).expect("Should not fail")
            })
    }
}

fn random_id() -> String {
    nanoid::nanoid!(6)
}

/// Returns the id.
///
/// Upserts the url into the database.
async fn insert_into_db(url: &str, pool: &PgPool) -> Result<String, UserError> {
    // Ensure url is a valid URL
    let url = Url::parse(url).map_err(|_| UserError::InvalidUrl)?;

    let id = random_id();

    // returning upsert: https://stackoverflow.com/a/37543015
    let (id,) = sqlx::query_as(
        "
    INSERT INTO urls(id, url)
    VALUES ($1, $2)
    ON CONFLICT (url)
    DO UPDATE SET
        url=EXCLUDED.url
    RETURNING id
    ",
    )
    .bind(&id)
    .bind(url.as_ref())
    .fetch_one(pool)
    .await
    .map_err(|err| UserError::Other(err.to_string()))?;

    Ok(id)
}

async fn get_from_db(id: &str, pool: &PgPool) -> Result<String, UserError> {
    let (url,) = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?
        .ok_or(UserError::NotFound)?;

    Ok(url)
}

#[get("/api/all")]
async fn display_all(state: web::Data<AppState>) -> Result<impl Responder> {
    let db = sqlx::query_as::<_, (String, String)>("SELECT * FROM urls")
        .fetch_all(&state.pool)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?
        .into_iter()
        .collect::<HashMap<_, _>>();

    Ok(web::Json(db))
}

// TODO?: change url path to /shorten
#[post("/api")]
async fn shorten_url(
    body: web::Json<types::ShortenRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    info!("shortening url: {}", body.url);

    let id = insert_into_db(&body.url, &state.pool).await?;

    Ok(web::Json(types::ShortenResponse { id }))
}

// TODO?: change url path
#[get("/api/{id}")]
async fn lengthen_url(
    path: web::Path<types::LengthenRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    info!("lengthening id: {}", path.id);

    let url = get_from_db(&path.id, &state.pool).await?;

    Ok(web::Json(types::LengthenResponse { url }))
}

/// Serve static files for Yew frontend under `/yew/{mount_path}`
///
/// Note: this won't work if using `cargo shuttle run` at repo top level.
///
/// https://actix.rs/docs/static-files
///
/// https://yew.rs/docs/more/deployment
fn yew_app(mount_path: &str, serve_from: &PathBuf) -> Files {
    use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};

    // Using a default handler to always show the Yew app
    // see https://yew.rs/docs/more/deployment#serving-indexhtml-as-fallback
    let default_handler = |req: ServiceRequest| async {
        let state: &web::Data<AppState> = req.app_data().expect("App data not set");

        let yew_folder = state.static_folder.join("yew");
        let index_file = yew_folder.join("index.html");

        let index_file = NamedFile::open_async(index_file).await?;

        let (req, _) = req.into_parts();
        let res = index_file.into_response(&req);
        Ok(ServiceResponse::new(req, res))
    };

    Files::new(mount_path, serve_from)
        .index_file("index.html")
        .redirect_to_slash_directory()
        .default_handler(fn_service(default_handler))
}

mod middleware {
    use actix_cors::Cors;
    use actix_web::middleware::{Logger, NormalizePath};

    pub fn cors() -> Cors {
        Cors::permissive()
    }

    pub fn logger() -> Logger {
        // TODO: configure logger, it's too verbose
        Logger::default()
    }

    pub fn normalize_path() -> NormalizePath {
        NormalizePath::trim()
    }
}

#[derive(Debug)]
struct AppState {
    pool: PgPool,
    static_folder: PathBuf,
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    info!("Running database migration");
    // TODO: use sqlx::migrate
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let yew_folder = static_folder.join("yew");

    let state = web::Data::new(AppState {
        pool,
        static_folder,
    });

    let config = move |cfg: &mut ServiceConfig| {
        let api_service = web::scope("")
            .wrap(middleware::normalize_path())
            .service(display_all)
            .service(shorten_url)
            .service(lengthen_url)
            .service(web::redirect("/", "/api/all"));

        let yew_service = web::scope("/yew").service(yew_app("/", &yew_folder));

        cfg.app_data(state).service(
            web::scope("")
                .wrap(middleware::cors())
                .wrap(middleware::logger())
                .service(yew_service)
                .service(api_service),
        );
    };

    Ok(config.into())
}
