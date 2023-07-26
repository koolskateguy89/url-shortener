use common::error::UrlError;
use common::types::{AllUrlsResponse, ErrorResponse, ShortenRequest, ShortenResponse};
use reqwest::{Error, Response, Result as ReqResult};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum ApiError {
    Url(UrlError),
    Reqwest(Error),
    Other(String),
}
pub type ApiResult<T> = Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        Self::Reqwest(error)
    }
}

fn api_url() -> String {
    // TODO: probably error handle
    std::env::var("API_URL").unwrap_or("http://localhost:8000/api".to_string())
}

async fn to_result<T>(response: Response) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    let status = response.status();

    if status.is_success() {
        let res = response.json().await?;
        Ok(res)
    } else if status.is_client_error() {
        let res = response.json::<ErrorResponse<UrlError>>().await?;
        Err(ApiError::Url(res.error))
    } else {
        // server error or network error(?)
        Err(ApiError::Other(response.text().await?))
    }
}

pub async fn get_all_urls() -> ReqResult<AllUrlsResponse> {
    let response = reqwest::get(format!("{}/urls", api_url())).await?;
    response.json().await
}

pub async fn shorten(url: String) -> ApiResult<ShortenResponse> {
    let body = ShortenRequest { url };

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/url/shorten", api_url()))
        .json(&body)
        .send()
        .await?;

    to_result(response).await
}

// TODO: lengthen
pub async fn lengthen(id: String) -> ApiResult<String> {
    Ok("lengthened".to_string())
}

// TODO: stats
pub async fn stats(id: String) -> ApiResult<String> {
    Ok("stats".to_string())
}
