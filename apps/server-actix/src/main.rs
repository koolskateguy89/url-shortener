use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error,
    http::{header::ContentType, StatusCode},
    web::{self, ServiceConfig},
    HttpResponse, Result,
};
use common::{
    error::{AuthError, Error, UrlError},
    types::ErrorResponse,
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::path::PathBuf;

mod config;
mod db;
mod middleware;
mod services;

use crate::services::{api::ApiService, yew::YewService};

#[derive(Debug)]
pub struct UserError(Error);

impl UserError {
    pub fn new(error: Error) -> Self {
        Self(error)
    }

    pub fn url(error: UrlError) -> Self {
        Self(Error::Url(error))
    }

    pub fn auth(error: AuthError) -> Self {
        Self(Error::Auth(error))
    }

    pub fn other(error: impl Into<String>) -> Self {
        Self(Error::Other(error.into()))
    }

    pub fn internal() -> Self {
        Self(Error::InternalError)
    }
}

// necessary for ResponseError
impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // debug of inner
        write!(f, "{:?}", self.0)
    }
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            Error::Url(UrlError::InvalidUrl) | Error::Auth(AuthError::InvalidCredentials) => {
                StatusCode::BAD_REQUEST
            }
            Error::Url(UrlError::NotFound) | Error::Auth(AuthError::UserNotFound) => {
                StatusCode::NOT_FOUND
            }
            Error::Auth(AuthError::UsernameTaken) => StatusCode::CONFLICT,
            Error::Auth(AuthError::UserIncorrectPassword) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorResponse::new(self.0.clone()))
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

async fn _get_and_delete_all_tables(pool: &PgPool) -> Result<(), shuttle_runtime::Error> {
    use sqlx::Executor;

    // get table names
    let table_names = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public'
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(shuttle_runtime::CustomError::new)?
    .into_iter()
    .map(|(table_name,)| table_name)
    .collect::<Vec<_>>();

    log::error!("table_names = {:?}", table_names);

    // delete all tables
    pool.execute(
        table_names
            .iter()
            .map(|table_name| format!("DROP TABLE IF EXISTS {} CASCADE", table_name))
            .collect::<Vec<_>>()
            .join(";")
            .as_str(),
    )
    .await
    .map_err(shuttle_runtime::CustomError::new)?;

    log::error!("done");

    Ok(())
}

/// Shuttle deployment breaks if we try to run migrations on it
/// so just have to do it manually, locally on deployment db, using
/// sqlx-cli
async fn _migrate(pool: &PgPool) -> Result<(), shuttle_runtime::Error> {
    log::info!("Running database migration");

    sqlx::migrate!()
        .run(pool)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(())
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // _get_and_delete_all_tables(&pool).await?;
    // _migrate(&pool).await?;

    {
        use config::auth::hash_password;
        use db::auth::{create_user, User};
        use log::{debug, error};

        if let Some(hashed_password) = hash_password("testpw".as_bytes()) {
            let test_user = User {
                username: "test".to_string(),
                hashed_password,
            };

            match create_user(&pool, &test_user).await {
                Ok(true) => debug!("test user created"),
                Ok(false) => debug!("test user already exists"),
                Err(sqlx_error) => {
                    error!("could not create test user, sqlx_error: {:?}", sqlx_error)
                }
            }
        } else {
            error!("could not hash test user password");
        }
    }

    let session_key = secret_store
        .get("SESSION_KEY")
        .ok_or_else(|| {
            shuttle_runtime::Error::BuildPanic("`SESSION_KEY` secret not set".to_string())
        })?
        .into_bytes();

    let config = move |cfg: &mut ServiceConfig| {
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
