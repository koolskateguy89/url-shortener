use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct User {
    pub username: String,
    pub hashed_password: String,
}

/// `Ok(true)` if user was created, `Ok(false)` if username was taken
pub async fn create_user(pool: &PgPool, user: &User) -> sqlx::Result<bool> {
    let (exists,): (bool,) =
        sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(&user.username)
            .fetch_one(pool)
            .await?;

    if exists {
        return Ok(false);
    }

    sqlx::query("INSERT INTO users(username, password) VALUES ($1, $2)")
        .bind(&user.username)
        .bind(&user.hashed_password)
        .execute(pool)
        .await?;

    Ok(true)
}

pub async fn get_user(pool: &PgPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as("SELECT username, password as hashed_password FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}
