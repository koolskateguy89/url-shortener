use common::types::ErrorResponse;
use gloo_net::http::Response;
use log::error;
use serde::de::DeserializeOwned;
use std::fmt::{Debug, Display};
use thiserror::Error;

pub mod auth;
pub mod url;

pub type NetResult<T> = Result<T, gloo_net::Error>;

fn to_error<E>(s: impl Display) -> ApiError<E> {
    ApiError::Other(format!("{s}"))
}

fn to_json_error<E>(e: impl Display) -> ApiError<E> {
    ApiError::Other(format!("json error: {e}"))
}

async fn error_from_response<E>(response: Response) -> ApiError<E>
where
    E: DeserializeOwned + Debug + Into<ApiError<E>>,
{
    let result = response
        .json::<ErrorResponse<E>>()
        .await
        .map_err(to_json_error)
        .map(|ErrorResponse { error }| {
            error!("error: {error:?}");

            // from generic E to ApiError<E>
            error.into()
        });

    match result {
        Ok(error_from_api) => error_from_api,
        Err(json_error) => json_error,
    }
}

/// Generic request status enum, not completely suitable for refetching
#[derive(Debug, Clone)]
pub enum RequestStatus<T, F> {
    Idle,
    Loading,
    Success(T),
    Error(F),
}

#[derive(Clone, Debug, Error)]
pub enum ApiError<E> {
    #[error(transparent)]
    Error(#[from] E),
    #[error("{0}")]
    Other(String),
}
