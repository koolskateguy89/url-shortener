use gloo_console::log;
use gloo_net::http::Request;

use std::fmt::Debug;

use common::{
    error::Error,
    types::{ErrorResponse, LengthenResponse, ShortenRequest, ShortenResponse},
};

fn to_error_other<T: Debug>(s: T) -> Error {
    Error::Other(format!("{s:?}"))
}

/// Make a request to the API to shorten a URL
pub async fn shorten(url: String) -> Result<ShortenResponse, Error> {
    let body = ShortenRequest { url };

    let response = Request::post("/api")
        .json(&body)
        .map_err(to_error_other)?
        .send()
        .await
        .map_err(to_error_other)?;

    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| Error::Other(format!("json error: {e}"))) // should not happen
    } else {
        let ErrorResponse { error } = response
            .json::<ErrorResponse>()
            .await
            .map_err(|e| Error::Other(format!("json error: {e}")))?;

        log!(format!("error: {error:?}"));

        Err(error)
    }
}

pub async fn lengthen(id: String) -> Result<LengthenResponse, Error> {
    let response = Request::get(format!("/api/{id}").as_str())
        .send()
        .await
        .map_err(to_error_other)?;

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
