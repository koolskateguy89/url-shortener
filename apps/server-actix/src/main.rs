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
use url::Url;

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use common::{
    error::Error,
    types::{self, ErrorResponse},
};

// TODO: serve static files for yew frontend under /yew
// https://actix.rs/docs/static-files
// https://yew.rs/docs/more/deployment

use derive_more::{Display, Error};

#[derive(Debug, Display, Error, Clone)]
pub enum UserError {
    #[display(fmt = "unused")]
    InvalidUrl,
    #[display(fmt = "unused")]
    NotFound,
    #[display(fmt = "unused")]
    InternalError,
}

impl From<UserError> for Error {
    fn from(e: UserError) -> Self {
        match e {
            UserError::InvalidUrl => Error::InvalidUrl,
            UserError::NotFound => Error::NotFound,
            _ => Error::Other(format!("{e}")),
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

/// returns the id
/// TODO?: upsert
fn insert_into_db(url: String, db: &mut HashMap<String, String>) -> Result<String> {
    // Ensure url is a valid URL
    Url::parse(&url).map_err(|_| UserError::InvalidUrl)?;

    // checks if url is already in db
    let id_in_db = db
        .iter()
        .find(|(k, v)| {
            if v == &&url {
                info!("found url in db: {k} -> {v}");
                true
            } else {
                false
            }
        })
        .map(|(k, _)| k);

    match id_in_db {
        Some(id) => Ok(id.clone()),
        None => {
            let id = random_id();
            info!("url not found in db, inserting: {id} -> {url}");
            db.insert(id.clone(), url.clone());
            Ok(id)
        }
    }
}

fn get_from_db<'a>(id: &String, db: &'a HashMap<String, String>) -> Option<&'a String> {
    db.get(id)
}

/// placeholder until using actual db
fn get_db(state: &web::Data<AppState>) -> Result<MutexGuard<HashMap<String, String>>> {
    state.db.lock().map_err(|_| UserError::InternalError.into())
}

#[get("/api/all")]
async fn display_all(state: web::Data<AppState>) -> Result<impl Responder> {
    let db = get_db(&state)?;
    Ok(web::Json(db.clone()))
}

// TODO?: change url path to /shorten
#[post("/api")]
async fn shorten_url(
    body: web::Json<types::ShortenRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    info!("shortening url: {}", body.url);

    let mut db = get_db(&state)?;
    let id = insert_into_db(body.url.clone(), &mut db)?;

    Ok(web::Json(types::ShortenResponse { id }))
}

// TODO?: rename func, will need to change url path possibly
// TODO?: change url path
#[get("/api/{id}")]
async fn lengthen_url(
    path: web::Path<types::LengthenRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let db = get_db(&state)?;
    let url = get_from_db(&path.id, &db)
        .cloned()
        .ok_or(UserError::NotFound)?;

    Ok(web::Json(types::LengthenResponse { url }))
}

#[derive(Debug)]
struct AppState {
    // pool: PgPool,
    /// id -> url
    db: Mutex<HashMap<String, String>>, // <- Mutex is necessary to mutate safely across threads
}

#[shuttle_runtime::main]
async fn actix_web(// TODO: db
    // #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // TODO: migration
    // pool.execute(include_str!("../schema.sql"))
    // .await
    // .map_err(CustomError::new)?;

    let state = web::Data::new(AppState {
        db: Mutex::default(),
    });

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
