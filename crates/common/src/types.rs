// TODO: data structs
// TODO: better names

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    pub url: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct LengthenRequest {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct LengthenResponse {
    pub url: String,
}
