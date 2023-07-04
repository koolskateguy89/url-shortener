use sqlx::PgPool;

use crate::auth::{hash_password, verify_password};

#[derive(Debug)]
pub enum LoginError {
    UserNotFound,
    IncorrectPassword,
    Sqlx(sqlx::Error),
}

#[derive(Debug)]
pub enum RegisterError {
    UsernameTaken,
    Sqlx(sqlx::Error),
}

impl From<sqlx::Error> for LoginError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}

impl From<sqlx::Error> for RegisterError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}

pub async fn register_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<(), RegisterError> {
    let (exists,): (bool,) =
        sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(username)
            .fetch_one(pool)
            .await?;

    if exists {
        return Err(RegisterError::UsernameTaken);
    }

    let hashed_password = hash_password(password.as_bytes());

    sqlx::query("INSERT INTO users(username, password) VALUES ($1, $2)")
        .bind(username)
        .bind(hashed_password)
        .execute(pool)
        .await?;

    Ok(())
}

/// `Ok` if user exists and password is correct
pub async fn can_login(pool: &PgPool, username: &str, password: &str) -> Result<(), LoginError> {
    let (hashed_password,) =
        sqlx::query_as::<_, (String,)>("SELECT password FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await?
            .ok_or(LoginError::UserNotFound)?;

    let correct_password = verify_password(password.as_bytes(), &hashed_password);

    if !correct_password {
        return Err(LoginError::IncorrectPassword);
    }

    Ok(())
}
