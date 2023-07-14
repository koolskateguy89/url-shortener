use common::{
    error::UrlError,
    types::{ErrorResponse, LengthenResponse, ShortenRequest, ShortenResponse, StatsResponse},
};
use gloo_net::http::Request;
use log::error;
use std::fmt::{Debug, Display};

use crate::api::ApiError;

fn to_error<E>(s: impl Debug) -> ApiError<E> {
    ApiError::Other(format!("{s:?}"))
}

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
        response
            .json()
            .await
            .map_err(|e| ApiError::Other(format!("json error: {e}"))) // should not happen
    } else {
        let ErrorResponse { error } = response
            .json::<ErrorResponse<UrlError>>()
            .await
            .map_err(|e| ApiError::Other(format!("json error: {e}")))?;

        error!("error: {error:?}");

        Err(error.into())
    }
}

pub async fn lengthen<T: Display>(id: T) -> Result<LengthenResponse, ApiError<UrlError>> {
    let response = Request::get(&format!("/api/url/{id}/lengthen"))
        .send()
        .await
        .map_err(to_error)?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| ApiError::Other(format!("json error: {e}"))) // should not happen
    } else {
        let ErrorResponse { error } = response
            .json::<ErrorResponse<UrlError>>()
            .await
            .map_err(|e| ApiError::Other(format!("json error: {e}")))?;

        error!("error: {error:?}");

        Err(error.into())
    }
}

pub async fn get_stats<T: Display>(id: T) -> Result<StatsResponse, ApiError<UrlError>> {
    let response = Request::get(&format!("/api/url/{id}/stats"))
        .send()
        .await
        .map_err(to_error)?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| ApiError::Other(format!("json error: {e}"))) // should not happen
    } else {
        let ErrorResponse { error } = response
            .json::<ErrorResponse<UrlError>>()
            .await
            .map_err(|e| ApiError::Other(format!("json error: {e}")))?;

        error!("error: {error:?}");

        Err(error.into())
    }
}
