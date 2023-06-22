use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: Error,
}

impl ErrorResponse {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}

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
