use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Error {
    InvalidUrl,
    NotFound,
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
