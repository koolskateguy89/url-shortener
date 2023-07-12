use actix_identity::Identity;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use common::{error::AuthError, types};

use crate::{config, db, AppState, UserError};
use config::auth::{hash_password, validate_credentials, verify_password};
use db::auth::User;

#[get("/api/users")]
pub async fn get_all_users(state: web::Data<AppState>) -> Result<impl Responder> {
    let usernames: Vec<String> = sqlx::query_as("SELECT username FROM users")
        .fetch_all(&state.pool)
        .await
        .map_err(|err| UserError::other(err.to_string()))?
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
    let types::LoginRequest { username, password } = body.into_inner();

    let result = db::auth::get_user(&state.pool, &username).await;

    match result {
        Ok(Some(user)) => {
            let correct_password = verify_password(password.as_bytes(), &user.hashed_password);

            if correct_password {
                log::debug!("login successful for `{}`", user.username);

                Identity::login(&req.extensions(), user.username).unwrap();
                Ok(HttpResponse::Ok().finish())
            } else {
                log::debug!("incorrect password for user `{username}`");
                Err(UserError::auth(AuthError::UserIncorrectPassword))?
            }
        }
        Ok(None) => {
            log::debug!("user `{username}` not found");
            Err(UserError::auth(AuthError::UserNotFound))?
        }
        Err(sqlx_error) => {
            log::warn!("sqlx error: {sqlx_error:?}");
            Err(UserError::internal())?
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
    let types::RegisterRequest { username, password } = body.into_inner();

    if !validate_credentials(&username, &password) {
        Err(UserError::auth(AuthError::InvalidCredentials))?
    }

    let hashed_password = hash_password(password.as_bytes()).ok_or(UserError::internal())?;

    let result = db::auth::create_user(
        &state.pool,
        &User {
            username: username.clone(),
            hashed_password,
        },
    )
    .await;

    match result {
        Ok(true) => {
            log::debug!("registration successful for `{}`", username);
            Ok(HttpResponse::Ok().finish())
        }
        Ok(false) => {
            log::debug!("tried to register already taken username `{username}`");
            Err(UserError::auth(AuthError::UsernameTaken))?
        }
        Err(sqlx_error) => {
            log::warn!("sqlx error: {sqlx_error:?}");
            Err(UserError::internal())?
        }
    }
}
