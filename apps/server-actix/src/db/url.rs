use common::error::UrlError;
use sqlx::{types::chrono, FromRow, PgPool};
use url::Url;

use crate::config::random_url_id;
use crate::UserError;

#[derive(Debug, FromRow)]
pub struct UrlRow {
    pub id: String,
    pub url: String,
    pub username: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
struct LengthenLogRow {
    id: String,
    date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct LengthenStat {
    pub url: String,
    pub username: Option<String>,
    pub hits: Vec<i64>,
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct UserRow {
    username: String,
    password: String,
}

impl LengthenStat {
    fn new(url: String, username: Option<String>, rows: Vec<LengthenLogRow>) -> Self {
        Self {
            url,
            username,
            hits: rows.into_iter().map(|row| row.date.timestamp()).collect(),
        }
    }
}

pub async fn get_all_urls(pool: &PgPool) -> sqlx::Result<Vec<UrlRow>> {
    sqlx::query_as("SELECT * FROM urls").fetch_all(pool).await
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
pub async fn insert_short_url(
    pool: &PgPool,
    url: &Url,
    username: Option<&str>,
) -> sqlx::Result<String> {
    let id = random_url_id();

    let id = if let Some(username) = username {
        // Unique constraint in the table ensures that the same username
        // cannot have the same url twice.

        // returning upsert: https://stackoverflow.com/a/37543015
        let (id,) = sqlx::query_as::<_, (String,)>(
            "
        INSERT INTO
          urls(id, url, username)
        VALUES
          ($1, $2, $3)
        ON CONFLICT (url, username)
        DO UPDATE SET
            url=EXCLUDED.url
        RETURNING id
        ",
        )
        .bind(&id)
        .bind(url.as_str())
        .bind(username)
        .fetch_one(pool)
        .await?;

        id
    } else {
        // But the unique constraint does not apply to null values. So when
        // username is none, have to explicitly check if url already exists
        let id_optional = sqlx::query_as::<_, (String,)>(
            "SELECT id from urls WHERE url = $1 AND username IS NULL",
        )
        .bind(url.as_str())
        .fetch_optional(pool)
        .await?;

        if let Some((id,)) = id_optional {
            id
        } else {
            // insert url without username
            sqlx::query("INSERT INTO urls(id, url) VALUES ($1, $2)")
                .bind(&id)
                .bind(url.as_str())
                .execute(pool)
                .await?;

            id
        }
    };

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
    let (url, username) = sqlx::query_as("SELECT url, username FROM urls WHERE id = $1")
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

    Ok(LengthenStat::new(url, username, rows))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_lengthen_stat() {
        let url = "https://example.com".to_string();
        let username = Some("username".to_string());

        let rows = vec![
            LengthenLogRow {
                id: "id".to_string(),
                date: Utc.timestamp_opt(1, 0).unwrap(),
            },
            LengthenLogRow {
                id: "id".to_string(),
                date: Utc.timestamp_opt(2, 0).unwrap(),
            },
        ];

        let stat = LengthenStat::new(url, username, rows);

        assert_eq!(stat.url, "https://example.com");
        assert_eq!(stat.username, Some("username".to_string()));
        assert_eq!(stat.hits, vec![1, 2]);
    }
}
