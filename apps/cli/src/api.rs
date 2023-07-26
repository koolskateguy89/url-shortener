use common::error::{self, UrlError};
use common::types::{self, ErrorResponse, ShortenRequest, ShortenResponse};
use reqwest::{Error, Response, Result as ReqResult};
use serde::de::DeserializeOwned;

// TODO: make result, and probably need to make manual error type
// that also includes reqwest::Error
fn api_url() -> String {
    // TODO: probably error handle
    std::env::var("API_URL").unwrap_or("http://localhost:8000/api".to_string())
}

// TODO: ApiError type (enum that includes reqwest::Error)
async fn to_result<T, E>(response: Response) -> Result<T, E>
where
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    let status = response.status();

    if status.is_success() {
        let res = response.json().await.expect("TODO: error handle");
        Ok(res)
    } else if status.is_client_error() {
        let res = response
            .json::<ErrorResponse<E>>()
            .await
            .expect("TODO: error handle");
        Err(res.error)
    } else {
        // server error or network error(?)
        let res = response.text().await;
        todo!("api error using other")
    }
}

// TODO: use error wrapper in result
pub async fn shorten(url: String) -> Result<ShortenResponse, UrlError> {
    let body = ShortenRequest { url };

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/url/shorten", api_url()))
        .json(&body)
        .send()
        .await
        .expect("TODO: error handle");

    to_result(response).await
}

// TODO: lengthen
pub async fn lengthen(id: String) -> ReqResult<String> {
    Ok("lengthened".to_string())
}

// TODO: stats
pub async fn stats(id: String) -> ReqResult<String> {
    Ok("stats".to_string())
}
