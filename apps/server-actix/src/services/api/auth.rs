use actix_identity::Identity;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};

use common::types;

use crate::db;
use crate::{AppState, UserError};

#[get("/api/users")]
pub async fn get_all_users(state: web::Data<AppState>) -> Result<impl Responder> {
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
pub async fn whoami(user: Option<Identity>) -> impl Responder {
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
// it works in yew, but not solid not next
#[post("/api/login")]
pub async fn login(
    body: web::Json<types::LoginRequest>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder> {
    use db::auth::LoginError;

    let types::LoginRequest { username, password } = body.into_inner();

    let result = db::auth::can_login(&state.pool, &username, &password).await;

    match result {
        Ok(_) => {
            log::debug!("login successful for `{username}`");

            Identity::login(&req.extensions(), username).unwrap();
            Ok(HttpResponse::Ok().finish())
        }
        Err(LoginError::UserNotFound) => {
            log::debug!("user `{username}` not found");
            Err(UserError::UserNotFound.into())
        }
        Err(LoginError::IncorrectPassword) => {
            log::debug!("incorrect password for user `{username}`");
            Err(UserError::UserIncorrectPassword.into())
        }
        Err(LoginError::Sqlx(sqlx_error)) => {
            log::warn!("sqlx error: {sqlx_error:?}");
            Err(UserError::InternalError.into())
        }
    }
}

#[post("/api/logout")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

#[post("/api/register")]
pub async fn register(
    body: web::Json<types::RegisterRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    use db::auth::RegisterError;

    let types::RegisterRequest { username, password } = body.into_inner();

    let result = db::auth::register_user(&state.pool, &username, &password).await;

    match result {
        Ok(_) => {
            log::debug!("registration successful for `{username}`");
            Ok(HttpResponse::Ok().finish())
        }
        Err(RegisterError::UsernameTaken) => {
            log::debug!("tried to register already taken username `{username}`");
            Err(UserError::UsernameTaken.into())
        }
        Err(RegisterError::Sqlx(sqlx_error)) => {
            log::warn!("sqlx error: {sqlx_error:?}");
            Err(UserError::InternalError.into())
        }
    }
}
