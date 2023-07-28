use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Serialize, Error)]
#[serde(untagged)]
pub enum Error {
    #[error(transparent)]
    Url(#[from] UrlError),
    #[error(transparent)]
    Auth(#[from] AuthError),
    // common
    #[error("Internal error")]
    InternalError,
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, Error)]
pub enum UrlError {
    #[error("Invalid URL")]
    InvalidUrl,
    #[error("ID not found")]
    NotFound,
}

#[derive(Clone, Debug, Deserialize, Serialize, Error)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,
    #[error("Incorrect password")]
    UserIncorrectPassword,
    #[error("Username taken")]
    UsernameTaken,
    #[error("Invalid credentials")]
    InvalidCredentials,
}
