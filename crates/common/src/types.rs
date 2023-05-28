// TODO: data structs
// TODO: better names

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub url: String,
    pub id: String,
}
