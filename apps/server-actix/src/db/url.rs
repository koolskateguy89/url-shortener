use common::error::UrlError;
use sqlx::{types::chrono, FromRow, PgPool};
use url::Url;

use crate::config::random_url_id;
use crate::UserError;

#[derive(Debug, FromRow)]
#[allow(dead_code)]
struct UrlRow {
    id: String,
    url: String,
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
struct LengthenLogRow {
    id: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct LengthenStat {
    pub url: String,
    pub hits: Vec<i64>,
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct UserRow {
    username: String,
    password: String,
}

impl LengthenStat {
    fn new(url: String, rows: Vec<LengthenLogRow>) -> Self {
        Self {
            url,
            hits: rows
                .into_iter()
                .map(|row| row.created_at.timestamp())
                .collect(),
        }
    }
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
pub async fn insert_short_url(pool: &PgPool, url: &Url) -> sqlx::Result<String> {
    let id = random_url_id();

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
    .bind(url.as_str())
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn get_long_url(pool: &PgPool, id: &str) -> Result<String, UserError> {
    let (url,) = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|err| UserError::other(err.to_string()))?
        .ok_or(UserError::url(UrlError::NotFound))?;

    sqlx::query("INSERT INTO lengthen_logs(id) VALUES ($1)")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|err| UserError::other(err.to_string()))?;

    Ok(url)
}

pub async fn get_lengthen_stats(pool: &PgPool, id: &str) -> Result<LengthenStat, UserError> {
    let (url,): (String,) = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|err| UserError::other(err.to_string()))?
        .ok_or(UserError::url(UrlError::NotFound))?;

    let rows: Vec<LengthenLogRow> = sqlx::query_as("SELECT * FROM lengthen_logs WHERE id = $1")
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(|err| UserError::other(err.to_string()))?;

    Ok(LengthenStat::new(url, rows))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_lengthen_stat() {
        let url = "https://example.com".to_string();

        let rows = vec![
            LengthenLogRow {
                id: "id".to_string(),
                created_at: Utc.timestamp_opt(1, 0).unwrap(),
            },
            LengthenLogRow {
                id: "id".to_string(),
                created_at: Utc.timestamp_opt(2, 0).unwrap(),
            },
        ];

        let stat = LengthenStat::new(url, rows);

        assert_eq!(stat.url, "https://example.com");
        assert_eq!(stat.hits, vec![1, 2]);
    }
}
