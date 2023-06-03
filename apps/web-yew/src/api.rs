use gloo_net::{http::Request, Error};

use common::types::{ShortenRequest, ShortenResponse};

#[derive(Debug)]
pub struct ApiShortenResponse {
    pub remote_url: String,
    pub id: String,
}

/// Make a request to the API to shorten a URL
pub async fn shorten(url: String) -> Result<ApiShortenResponse, Error> {
    let body = ShortenRequest { url };

    let ShortenResponse { url, id } = Request::post("/api")
        .json(&body)?
        .send()
        .await?
        .json()
        .await?;

    Ok(ApiShortenResponse {
        remote_url: url,
        id,
    })
}
