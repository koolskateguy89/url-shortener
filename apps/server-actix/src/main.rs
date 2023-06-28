use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error,
    http::{header::ContentType, StatusCode},
    web::{self, ServiceConfig},
    HttpResponse, Result,
};
use derive_more::Display;
use log::info;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};

use std::path::PathBuf;

use common::{error::Error as CommonError, types::ErrorResponse};

mod db;
mod middleware;
mod services;

use crate::services::{api::ApiService, yew::YewService};

// https://discord.com/channels/803236282088161321/1122643649503694919
// Shuttle windows bug has been fixed, had to build from source
// cargo install cargo-shuttle --git=https://github.com/shuttle-hq/shuttle
// , will cargo-binstall when it's released

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

impl From<UserError> for CommonError {
    fn from(e: UserError) -> Self {
        match e {
            UserError::InvalidUrl => CommonError::InvalidUrl,
            UserError::NotFound => CommonError::NotFound,
            UserError::Other(s) => CommonError::Other(s),
            _ => CommonError::Other(format!("{e:?}")),
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

async fn not_found_handler(req: ServiceRequest) -> Result<ServiceResponse> {
    let res = HttpResponse::NotFound().body("not found");
    Ok(req.into_response(res))
}

#[derive(Debug)]
pub struct AppState {
    pool: PgPool,
    static_folder: PathBuf,
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    info!("Running database migration");
    // TODO: use sqlx::migrate
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let config = move |cfg: &mut ServiceConfig| {
        let session_key = secret_store
            .get("SESSION_KEY")
            .expect("`SESSION_KEY` secret not set")
            .into_bytes();

        let yew_folder = static_folder.join("yew");

        let state = web::Data::new(AppState {
            pool,
            static_folder,
        });

        cfg.app_data(state)
            .yew_service(yew_folder)
            .api_service(&session_key)
            .default_service(not_found_handler);
    };

    Ok(config.into())
}

// TODO: split tests into separate files

// #[cfg(test)]
// mod tests {
//     use actix_web::{http::header, test, App};

//     use super::*;

//     #[actix_web::test]
//     async fn test_index_redirect() {
//         let app = test::init_service(
//             App::new()
//                 .wrap(middleware::normalize_path())
//                 // .service(get_all_urls)
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
//     async fn test_get_all_urls() {
//         // TODO: app state pool - how?

//         let app = test::init_service(
//             App::new()
//                 .wrap(middleware::normalize_path())
//                 .service(get_all_urls),
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
