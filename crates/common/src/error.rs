use serde::{Deserialize, Serialize};

// TODO?: rename to UrlError, it's used in a lot of places
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Error {
    InvalidUrl,
    NotFound,
    Other(String),
}

pub type UrlResult<T> = std::result::Result<T, Error>;

// TODO: auth related errors
