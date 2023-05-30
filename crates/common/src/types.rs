use serde::{Deserialize, Serialize};

// TODO?: better names

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortenResponse {
    pub url: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LengthenRequest {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LengthenResponse {
    pub url: String,
}
