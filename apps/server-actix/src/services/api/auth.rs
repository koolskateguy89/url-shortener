use actix_identity::Identity;
use actix_web::{error, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};

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
#[post("/api/login")]
pub async fn login(
    body: web::Json<types::LoginRequest>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let types::LoginRequest { username, password } = body.into_inner();

    let user_exists = db::auth::user_exists(&state.pool, &username)
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
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

#[post("/api/register")]
pub async fn register(
    body: web::Json<types::RegisterRequest>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let types::RegisterRequest { username, password } = body.into_inner();

    // TODO: validate pw and if user exists

    Err::<String, _>(error::ErrorInternalServerError("not implemented"))
}
