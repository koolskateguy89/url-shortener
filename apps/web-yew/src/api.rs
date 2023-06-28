use gloo_console::log;
use gloo_net::http::Request;

use std::fmt::{Debug, Display};

use common::{
    error::{Error, Result},
    types::{
        ErrorResponse, LengthenResponse, LoginRequest, RegisterRequest, ShortenRequest,
        ShortenResponse, StatsResponse,
    },
};

fn to_error<T: Debug>(s: T) -> Error {
    Error::Other(format!("{s:?}"))
}

/// Make a request to the API to shorten a URL
pub async fn shorten(url: String) -> Result<ShortenResponse> {
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
    let response = Request::get(&format!("/api/url/{id}/lengthen"))
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
    let response = Request::get(&format!("/api/url/{id}/stats"))
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

pub async fn whoami() -> Result<String> {
    let response = Request::get("/api/whoami").send().await.map_err(to_error)?;

    response
        .text()
        .await
        .map_err(|e| Error::Other(format!("text error: {e}"))) // should not happen
}

// TODO?: login error enum for result
pub async fn login(username: impl Into<String>, password: impl Into<String>) -> Result<()> {
    let body = LoginRequest {
        username: username.into(),
        password: password.into(),
    };

    todo!()
}

pub async fn logout() -> Result<()> {
    todo!()
}

pub async fn register(username: impl Into<String>, password: impl Into<String>) -> Result<()> {
    let body = RegisterRequest {
        username: username.into(),
        password: password.into(),
    };

    todo!()
}
