use gloo_net::{http::Request, Error};

use common::types::{LengthenResponse, ShortenRequest, ShortenResponse};

/// Make a request to the API to shorten a URL
pub async fn shorten(url: String) -> Result<ShortenResponse, Error> {
    let body = ShortenRequest { url };

    Ok(Request::post("/api")
        .json(&body)?
        .send()
        .await?
        .json()
        .await?)
}

pub async fn lengthen(id: String) -> Result<LengthenResponse, Error> {
    Ok(Request::get(format!("/api/{id}").as_str())
        .send()
        .await?
        .json()
        .await?)
}
