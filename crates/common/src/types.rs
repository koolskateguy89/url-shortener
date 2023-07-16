use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse<E> {
    pub error: E,
}

impl<E> ErrorResponse<E> {
    pub fn new(error: E) -> Self {
        Self { error }
    }
}

// Url shortening

#[derive(Debug, Deserialize, Serialize)]
pub struct UrlInfo {
    pub id: String,
    pub url: String,
    pub username: Option<String>,
    pub created_at: i64,
}
pub type AllUrlsResponse = HashMap<String, UrlInfo>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortenResponse {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LengthenResponse {
    pub url: String,
}

/// TODO: more stats ig
#[derive(Debug, Deserialize, Serialize)]
pub struct StatsResponse {
    pub url: String,
    pub num_hits: usize,
    pub hits: Vec<i64>,
}

// Auth

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}
