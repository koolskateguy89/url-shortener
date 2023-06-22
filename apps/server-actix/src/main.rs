use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    error, get,
    http::{header::ContentType, StatusCode},
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder, Result,
};
use derive_more::Display;
use log::info;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

use std::collections::HashMap;
use std::path::PathBuf;

use common::{
    error::Error,
    types::{self, ErrorResponse},
};

mod db;

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
            .json(ErrorResponse::new(self.clone().into()))
    }
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

    let id = db::insert_short_url(&state.pool, &body.url).await?;

    Ok(web::Json(types::ShortenResponse { id }))
}

// TODO?: change url path
#[get("/api/{id}")]
async fn lengthen_url(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let (id,) = path.into_inner();

    info!("lengthening id: {}", id);

    let url = db::get_long_url(&state.pool, &id).await?;

    Ok(web::Json(types::LengthenResponse { url }))
}

#[get("/api/{id}/exists")]
async fn id_exists(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let (id,) = path.into_inner();

    let exists = db::id_exists(&state.pool, &id)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?;

    match exists {
        true => Ok("exists"),
        false => Err(UserError::NotFound.into()),
    }
}

#[get("/api/{id}/stats")]
async fn lengthen_stats(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let (id,) = path.into_inner();

    let db::LengthenStat { url, hits } = db::get_lengthen_stats(&state.pool, &id).await?;

    Ok(web::Json(types::StatsResponse {
        url,
        num_hits: hits.len(),
        hits,
    }))
}

/// Serve static files for Yew frontend under `/yew/{mount_path}`
///
/// Note: this won't work if using `cargo shuttle run` at repo top level.
///
/// https://actix.rs/docs/static-files
///
/// https://yew.rs/docs/more/deployment
fn yew_app<T: Into<PathBuf>>(mount_path: &str, serve_from: T) -> Files {
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

async fn not_found_handler(req: ServiceRequest) -> Result<ServiceResponse> {
    let (req, _) = req.into_parts();
    let res = HttpResponse::NotFound().body("not found");
    Ok(ServiceResponse::new(req, res))
}

mod middleware {
    use actix_cors::Cors;
    use actix_web::middleware::{Logger, NormalizePath};

    pub fn cors() -> Cors {
        Cors::permissive()
    }

    pub fn logger() -> Logger {
        Logger::new(r#"%a "%r" %s, elapsed: %Dms"#)
    }

    pub fn normalize_path() -> NormalizePath {
        NormalizePath::trim()
    }
}

trait Services {
    fn api_service(&mut self) -> &mut Self;
    fn yew_service<T: Into<PathBuf>>(&mut self, yew_folder: T) -> &mut Self;
}

impl Services for ServiceConfig {
    fn api_service(&mut self) -> &mut Self {
        self.service(
            web::scope("")
                .wrap(middleware::cors())
                .wrap(middleware::logger())
                .wrap(middleware::normalize_path())
                .service(display_all)
                .service(shorten_url)
                .service(lengthen_url)
                .service(lengthen_stats)
                .service(id_exists)
                .service(web::redirect("/", "/api/all")),
        )
    }

    fn yew_service<T: Into<PathBuf>>(&mut self, yew_folder: T) -> &mut Self {
        self.service(
            web::scope("/yew")
                .wrap(middleware::logger())
                .service(yew_app("/", yew_folder)),
        )
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

    let config = move |cfg: &mut ServiceConfig| {
        let yew_folder = static_folder.join("yew");

        let state = web::Data::new(AppState {
            pool,
            static_folder,
        });

        cfg.app_data(state)
            .yew_service(yew_folder)
            .api_service()
            .default_service(not_found_handler);
    };

    Ok(config.into())
}

// #[cfg(test)]
// mod tests {
//     use actix_web::{http::header, test, App};

//     use super::*;

//     #[actix_web::test]
//     async fn test_index_redirect() {
//         let app = test::init_service(
//             App::new()
//                 .wrap(middleware::normalize_path())
//                 // .service(display_all)
//                 .service(web::redirect("/", "/api/all")),
//         )
//         .await;

//         let req = test::TestRequest::get().uri("/").to_request();
//         let resp = test::call_service(&app, req).await;

//         assert!(resp.status().is_redirection());
//         assert!(resp.headers().contains_key(header::LOCATION));
//         assert_eq!(
//             resp.headers()
//                 .get(header::LOCATION)
//                 .expect("Location header not set"),
//             "/api/all"
//         )
//     }

//     #[actix_web::test]
//     async fn test_display_all() {
//         // TODO: app state pool - how?

//         let app = test::init_service(
//             App::new()
//                 .wrap(middleware::normalize_path())
//                 .service(display_all),
//         )
//         .await;

//         let req = test::TestRequest::get().uri("/api/all").to_request();
//         let resp = test::call_service(&app, req).await;

//         println!("resp = [{:?}]", resp);

//         assert!(resp.status().is_success());
//     }

//     #[actix_web::test]
//     async fn test_shorten_url() {
//         // TODO: app state pool - how?

//         let app = test::init_service(
//             App::new()
//                 .wrap(middleware::normalize_path())
//                 .service(shorten_url),
//         )
//         .await;

//         let req = test::TestRequest::post()
//             .uri("/api")
//             .set_json(types::ShortenRequest {
//                 url: "https://google.com".to_string(),
//             })
//             .to_request();
//         let resp = test::call_service(&app, req).await;

//         println!("resp = [{:?}]", resp);

//         assert!(resp.status().is_success());
//         assert_eq!(
//             resp.headers()
//                 .get(header::CONTENT_TYPE)
//                 .expect("Content type not set"),
//             header::ContentType::json().to_string().as_str()
//         );
//         // TODO: assert body is valid json
//         // TODO: assert body is valid ShortenResponse
//     }
// }
