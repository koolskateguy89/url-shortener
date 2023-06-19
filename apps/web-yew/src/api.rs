use gloo_console::log;
use gloo_net::http::Request;

use std::fmt::{Debug, Display};

use common::{
    error::{Error, Result},
    types::{ErrorResponse, LengthenResponse, ShortenRequest, ShortenResponse, StatsResponse},
};

fn to_error<T: Debug>(s: T) -> Error {
    Error::Other(format!("{s:?}"))
}

/// Make a request to the API to shorten a URL
pub async fn shorten(url: String) -> Result<ShortenResponse> {
    let body = ShortenRequest { url };

    let response = Request::post("/api")
        .json(&body)
        .map_err(to_error)? // should not happen
        .send()
        .await
        .map_err(to_error)?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| Error::Other(format!("json error: {e}"))) // should not happen
    } else {
        let ErrorResponse { error } = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("json error: {e}")))?;

        log!(format!("error: {error:?}"));

        Err(error)
    }
}

pub async fn lengthen<T: Display>(id: T) -> Result<LengthenResponse> {
    let response = Request::get(&format!("/api/{id}"))
        .send()
        .await
        .map_err(to_error)?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| Error::Other(format!("json error: {e}"))) // should not happen
    } else {
        let ErrorResponse { error } = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("json error: {e}")))?;

        log!(format!("error: {error:?}"));

        Err(error)
    }
}

pub async fn get_stats<T: Display>(id: T) -> Result<StatsResponse> {
    let response = Request::get(&format!("/api/{id}/stats"))
        .send()
        .await
        .map_err(to_error)?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| Error::Other(format!("json error: {e}"))) // should not happen
    } else {
        let ErrorResponse { error } = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("json error: {e}")))?;

        log!(format!("error: {error:?}"));

        Err(error)
    }
}
