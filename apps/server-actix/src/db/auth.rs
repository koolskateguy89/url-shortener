use sqlx::PgPool;

pub async fn user_exists(pool: &PgPool, username: &str) -> sqlx::Result<bool> {
    let (exists,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(exists)
}

// TODO: find password hashing lib
