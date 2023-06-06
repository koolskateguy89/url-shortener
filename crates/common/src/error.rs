use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Error {
    InvalidUrl,
    NotFound,
    Other(String),
}
