use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Error {
    Url(UrlError),
    Auth(AuthError),
    // common
    InternalError,
    Other(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UrlError {
    InvalidUrl,
    NotFound,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AuthError {
    UserNotFound,
    UserIncorrectPassword,
    UsernameTaken,
    InvalidCredentials,
}

impl From<UrlError> for Error {
    fn from(e: UrlError) -> Self {
        Self::Url(e)
    }
}

impl From<AuthError> for Error {
    fn from(e: AuthError) -> Self {
        Self::Auth(e)
    }
}
