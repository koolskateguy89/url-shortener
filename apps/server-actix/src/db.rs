use sqlx::PgPool;
use url::Url;

use crate::UserError;

fn random_id() -> String {
    nanoid::nanoid!(6)
}

pub async fn id_exists(pool: &PgPool, id: &str) -> sqlx::Result<bool> {
    let (exists,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM urls WHERE id = $1)")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(exists)
}

/// Returns the `id`.
///
/// Upserts the url into the database.
pub async fn insert_short_url(pool: &PgPool, url: &str) -> Result<String, UserError> {
    // Ensure url is a valid URL
    let url = Url::parse(url).map_err(|_| UserError::InvalidUrl)?;

    let id = random_id();

    // returning upsert: https://stackoverflow.com/a/37543015
    let (id,) = sqlx::query_as(
        "
    INSERT INTO
      urls(id, url)
    VALUES
      ($1, $2)
    ON CONFLICT (url)
    DO UPDATE SET
        url=EXCLUDED.url
    RETURNING id
    ",
    )
    .bind(&id)
    .bind(url.as_ref())
    .fetch_one(pool)
    .await
    .map_err(|err| UserError::Other(err.to_string()))?;

    Ok(id)
}

pub async fn get_long_url(pool: &PgPool, id: &str) -> Result<String, UserError> {
    let (url,) = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?
        .ok_or(UserError::NotFound)?;

    sqlx::query("INSERT INTO lengthen_logs(id) VALUES ($1)")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?;

    Ok(url)
}

pub async fn get_lengthen_stats(pool: &PgPool, id: &str) -> Result<(String, i64), UserError> {
    sqlx::query_as(
        "
    SELECT
      urls.url,
      count(lengthen_logs.id)
    from
      urls
      LEFT JOIN lengthen_logs ON urls.id = lengthen_logs.id
    WHERE
      urls.id = $1
    GROUP BY
      urls.url
    ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|err| UserError::Other(err.to_string()))?
    .ok_or(UserError::NotFound)
}
