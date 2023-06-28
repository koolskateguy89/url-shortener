use actix_files::{Directory, Files, NamedFile};
use actix_identity::Identity;
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    error, get,
    http::{header::ContentType, StatusCode},
    post,
    web::{self, ServiceConfig},
    HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};
use derive_more::Display;
use log::info;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};

use std::collections::HashMap;
use std::path::PathBuf;

use common::{
    error::Error as CommonError,
    types::{self, ErrorResponse},
};

mod db;
mod middleware;

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

#[get("/api/urls")]
async fn get_all_urls(state: web::Data<AppState>) -> Result<impl Responder> {
    let db = sqlx::query_as::<_, (String, String)>("SELECT * FROM urls")
        .fetch_all(&state.pool)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?
        .into_iter()
        .collect::<HashMap<_, _>>();

    Ok(web::Json(db))
}

#[post("/api/url/shorten")]
async fn shorten_url(
    body: web::Json<types::ShortenRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    info!("shortening url: {}", body.url);

    let id = db::insert_short_url(&state.pool, &body.url).await?;

    Ok(web::Json(types::ShortenResponse { id }))
}

#[get("/api/url/{id}/lengthen")]
async fn lengthen_url(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let (id,) = path.into_inner();

    info!("lengthening id: {}", id);

    let url = db::get_long_url(&state.pool, &id).await?;

    Ok(web::Json(types::LengthenResponse { url }))
}

#[get("/api/url/{id}/exists")]
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

#[get("/api/url/{id}/stats")]
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

#[get("/api/users")]
async fn get_all_users(state: web::Data<AppState>) -> Result<impl Responder> {
    let usernames: Vec<String> = sqlx::query_as("SELECT username FROM users")
        .fetch_all(&state.pool)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?
        .into_iter()
        .map(|(username,)| username)
        .collect();

    Ok(web::Json(usernames))
}

#[get("/api/whoami")]
async fn whoami(user: Option<Identity>) -> impl Responder {
    let res = match user {
        Some(user) => user.id().unwrap(),
        None => "not logged in".to_string(),
    };

    log::warn!("whoami = {:?}", res);

    res
}

// FIXME: identity not working
// whoami still says not logged in after login DEFINITELY working
// Set-Cookie header is set in response but _shrug_
#[post("/api/login")]
async fn login(
    body: web::Json<types::LoginRequest>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let types::LoginRequest { username, password } = body.into_inner();

    let user_exists = db::user_exists(&state.pool, &username)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?;

    if !user_exists {
        log::warn!("user `{}` not found", username);
        return Err(UserError::NotFound.into());
    }

    // TODO: check pw correct

    // TODO: propagate error
    Identity::login(&req.extensions(), username).unwrap();

    Ok(HttpResponse::Ok().finish())
}

#[post("/api/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

#[post("/api/register")]
async fn register(
    body: web::Json<types::RegisterRequest>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let types::RegisterRequest { username, password } = body.into_inner();

    // TODO: validate pw and if user exists

    Err::<String, _>(error::ErrorInternalServerError("not implemented"))
}

/// Serve static files for Yew frontend under `/yew/{mount_path}`
///
/// Note: this won't work if using `cargo shuttle run` at repo top level.
///
/// https://actix.rs/docs/static-files
///
/// https://yew.rs/docs/more/deployment
fn yew_app(mount_path: &str, serve_from: impl Into<PathBuf>) -> Files {
    // Using a default handler to always show the Yew app
    // see https://yew.rs/docs/more/deployment#serving-indexhtml-as-fallback
    let default_handler = |req: ServiceRequest| async {
        let state: &web::Data<AppState> = req.app_data().expect("App data not set");

        let yew_folder = state.static_folder.join("yew");
        let index_file = yew_folder.join("index.html");

        let index_file = NamedFile::open_async(index_file).await?;

        let res = index_file.into_response(req.request());
        Ok(req.into_response(res))
    };

    // This is only really for home page (/yew/)
    // Use custom files listing renderer to always show the Yew app
    let dir_renderer = |dir: &Directory, req: &HttpRequest| {
        // /yew will show 404 in yew app
        // TODO: if path is /yew, redir to /yew/
        // but unforunately have no way to know if it's /yew,
        // - if url is /yew or /yew/,
        // `req` gives /yew
        // and `dir` doesn't help

        let index_file = NamedFile::open(dir.path.join("index.html"))?;
        let res = index_file.into_response(req);
        Ok(ServiceResponse::new(req.clone(), res))
    };

    Files::new(mount_path, serve_from)
        // .index_file("index.html")
        // .redirect_to_slash_directory()
        .prefer_utf8(true)
        .show_files_listing()
        .files_listing_renderer(dir_renderer)
        .default_handler(fn_service(default_handler))
}

async fn not_found_handler(req: ServiceRequest) -> Result<ServiceResponse> {
    let res = HttpResponse::NotFound().body("not found");
    Ok(req.into_response(res))
}

trait Services {
    fn api_service(&mut self, session_key: &[u8]) -> &mut Self;
    fn yew_service(&mut self, yew_folder: impl Into<PathBuf>) -> &mut Self;
}

impl Services for ServiceConfig {
    fn api_service(&mut self, session_key: &[u8]) -> &mut Self {
        self.service(
            web::scope("")
                .wrap(middleware::cors())
                .wrap(middleware::logger())
                .wrap(middleware::normalize_path())
                .wrap(middleware::identity())
                .wrap(middleware::session(session_key))
                // url shortener
                .service(get_all_urls)
                .service(web::redirect("/", "/api/urls"))
                .service(shorten_url)
                .service(lengthen_url)
                .service(lengthen_stats)
                .service(id_exists)
                // user auth
                .service(get_all_users)
                .service(whoami)
                .service(login)
                .service(logout)
                .service(register),
        )
    }

    fn yew_service(&mut self, yew_folder: impl Into<PathBuf>) -> &mut Self {
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
