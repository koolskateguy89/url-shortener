use actix_web::web::{self, ServiceConfig};

use crate::middleware;

mod auth;
mod url;

use self::url::*;
use auth::*;

pub trait ApiService {
    fn api_service(&mut self, session_key: &[u8]) -> &mut Self;
}

impl ApiService for ServiceConfig {
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
}
