use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{
    error, get, post,
    web::{self, ServiceConfig},
    Responder, Result,
};
use log::info;
use shuttle_actix_web::ShuttleActixWeb;

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use common::types;

// TODO: make it serve on 0.0.0.0 instead of 127.0.0.1

fn random_id() -> String {
    use nanoid::nanoid;
    nanoid!(6)
}

/// returns the id
/// TODO?: upsert
fn insert_into_db(url: String, db: &mut HashMap<String, String>) -> Result<String> {
    // TODO: validate url is valid -> bad request err if not

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

fn get_from_db<'a>(id: &'a String, db: &'a HashMap<String, String>) -> Option<&'a String> {
    db.get(id)
}

/// placeholder until using actual db
fn get_db(state: &web::Data<AppState>) -> Result<MutexGuard<HashMap<String, String>>> {
    state
        .db
        .lock()
        .map_err(|_| error::ErrorInternalServerError("Failed to get db"))
}

#[get("/all")]
async fn display_all(state: web::Data<AppState>) -> Result<String> {
    let db = get_db(&state)?;
    Ok(format!("{:?}", db))
}

#[post("/")]
async fn shorten_url(
    body: web::Json<types::ShortenRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    info!("shortening url: {}", body.url);

    let mut db = get_db(&state)?;
    let id = insert_into_db(body.url.clone(), &mut db)?;

    // TODO: get URL from config (or env var)
    Ok(web::Json(types::ShortenResponse {
        url: format!("http://localhost:8000/{id}"),
        id,
    }))
}

// // TODO?: rename func, will need to change url path possibly
#[get("/{id}")]
async fn lengthen_url(
    path: web::Path<types::LengthenRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let db = get_db(&state)?;
    let url = get_from_db(&path.id, &db);

    match url {
        Some(url) => Ok(web::Json(types::LengthenResponse { url: url.clone() })),
        None => Err(error::ErrorNotFound("id not found")),
    }
}

// TODO: a redirect endpoint

#[derive(Debug)]
struct AppState {
    /// id -> url
    db: Mutex<HashMap<String, String>>, // <- Mutex is necessary to mutate safely across threads
                                        // pool: PgPool,
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
                .service(display_all)
                .service(shorten_url)
                .service(lengthen_url),
        );
    };

    Ok(config.into())
}
