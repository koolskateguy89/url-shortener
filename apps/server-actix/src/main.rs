use actix_cors::Cors;
use actix_web::middleware::{Logger, NormalizePath};
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

use common::{
    error::Error,
    types::{self, ErrorResponse},
};

// TODO: serve static files for yew frontend under /yew
// https://actix.rs/docs/static-files
// https://yew.rs/docs/more/deployment

use derive_more::Display;

#[derive(Debug)]
struct AppState {
    pool: PgPool,
}

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
async fn insert_into_db(url: &str, pool: &PgPool) -> Result<String> {
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

async fn get_from_db(id: &String, pool: &PgPool) -> Result<String> {
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

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::permissive();

        cfg.app_data(state).service(
            web::scope("")
                .wrap(cors)
                .wrap(Logger::default())
                .wrap(NormalizePath::trim())
                .service(display_all)
                .service(shorten_url)
                .service(lengthen_url)
                .service(web::redirect("/", "/api/all")),
        );
    };

    Ok(config.into())
}
