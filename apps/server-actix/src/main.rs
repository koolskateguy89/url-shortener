use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    Responder, Result,
};
use log::debug;
use shuttle_actix_web::ShuttleActixWeb;

use common::types;

// TODO: make it serve on 0.0.0.0 instead of 127.0.0.1

// TODO: store a map of id -> url and a map of url -> id
// in app state (https://actix.rs/docs/application#shared-mutable-state)
// until database is done (once decided on ORM etc.)

fn random_id() -> String {
    use nanoid::nanoid;
    nanoid!(6)
}

#[get("/")]
async fn hello_world() -> String {
    println!("hello world");

    format!("(server-actix) Hello World! Id: {}", random_id())
}

#[post("/")]
async fn shorten_url(body: web::Json<types::ShortenRequest>) -> impl Responder {
    debug!("Shortening url: {}", body.url);

    // TODO: check if url is valid
    // TODO: insert into db (state for now)

    web::Json(types::ShortenResponse {
        url: body.url.clone(),
        id: random_id(),
    })
}

// // TODO?: rename func, will need to change url path possibly
#[get("/{id}")]
async fn lengthen_url(path: web::Path<types::LengthenRequest>) -> Result<impl Responder> {
    // TODO: get from db (state for now)
    Ok(format!("id: {}", path.id))
}

// TODO: a redirect endpoint

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::permissive();

        cfg.service(
            web::scope("")
                .wrap(cors)
                .wrap(Logger::default())
                .service(hello_world)
                .service(shorten_url)
                .service(lengthen_url),
        );
    };

    Ok(config.into())
}
