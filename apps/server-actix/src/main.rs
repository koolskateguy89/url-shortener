use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

// TODO: make it serve on 0.0.0.0 instead of 127.0.0.1

#[get("/")]
async fn hello_world() -> &'static str {
    "(server-actix) Hello World!"
}

#[shuttle_runtime::main]
#[rustfmt::skip]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}
