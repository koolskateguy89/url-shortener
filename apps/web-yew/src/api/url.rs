use common::{
    error::UrlError,
    types::{LengthenResponse, ShortenRequest, ShortenResponse, StatsResponse},
};
use gloo_net::http::Request;
use std::fmt::Display;

use super::{error_from_response, to_error, to_json_error, ApiError};

/// Make a request to the API to shorten a URL
pub async fn shorten(url: String) -> Result<ShortenResponse, ApiError<UrlError>> {
    let body = ShortenRequest { url };

    let response = Request::post("/api/url/shorten")
        .json(&body)
        .map_err(to_error)? // should not happen
        .send()
        .await
        .map_err(to_error)?;

    if response.ok() {
        response.json().await.map_err(to_json_error) // should not happen
    } else {
        Err(error_from_response(response).await)
    }
}

pub async fn lengthen<T: Display>(id: T) -> Result<LengthenResponse, ApiError<UrlError>> {
    let response = Request::get(&format!("/api/url/{id}/lengthen"))
        .send()
        .await
        .map_err(to_error)?;

    if response.ok() {
        response.json().await.map_err(to_json_error) // should not happen
    } else {
        Err(error_from_response(response).await)
    }
}

pub async fn get_stats<T: Display>(id: T) -> Result<StatsResponse, ApiError<UrlError>> {
    let response = Request::get(&format!("/api/url/{id}/stats"))
        .send()
        .await
        .map_err(to_error)?;

    if response.ok() {
        response.json().await.map_err(to_json_error) // should not happen
    } else {
        Err(error_from_response(response).await)
    }
}
