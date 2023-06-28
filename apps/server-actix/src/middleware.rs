use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::{Logger, NormalizePath},
};

pub fn cors() -> Cors {
    Cors::permissive()
}

pub fn logger() -> Logger {
    Logger::new(r#"%a "%r" %s, elapsed: %Dms"#)
}

pub fn normalize_path() -> NormalizePath {
    NormalizePath::trim()
}

pub fn session(session_key: &[u8]) -> SessionMiddleware<CookieSessionStore> {
    use actix_session::config::CookieContentSecurity;

    let cookie_store = CookieSessionStore::default();
    let key = Key::from(session_key);

    SessionMiddleware::builder(cookie_store, key)
        .cookie_content_security(CookieContentSecurity::Private)
        .build()
}

pub fn identity() -> IdentityMiddleware {
    IdentityMiddleware::default()
}
