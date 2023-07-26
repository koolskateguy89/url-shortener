use common::error::UrlError;
use common::types::{
    AllUrlsResponse, ErrorResponse, LengthenResponse, ShortenRequest, ShortenResponse,
    StatsResponse,
};
use reqwest::{Error, Response, Result as ReqResult, StatusCode};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum ApiError {
    Url(UrlError),
    Reqwest(Error),
    Other(StatusCode, String),
}
pub type ApiResult<T> = Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        Self::Reqwest(error)
    }
}

/// Basically `format!` but with API_URL prepended
macro_rules! api_url {
    ($($arg:tt)*) => {{
        // TODO: probably error handle
        let api_url = std::env::var("API_URL").unwrap_or("http://localhost:8000/api".to_string());
        let endpoint = format!($($arg)*);
        format!("{api_url}{endpoint}")
    }}
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
        Err(ApiError::Other(status, response.text().await?))
    }
}

pub async fn get_all_urls() -> ReqResult<AllUrlsResponse> {
    let response = reqwest::get(api_url!("/urls")).await?;
    response.json().await
}

pub async fn shorten(url: String) -> ApiResult<ShortenResponse> {
    let body = ShortenRequest { url };

    let client = reqwest::Client::new();
    let response = client
        .post(api_url!("/url/shorten"))
        .json(&body)
        .send()
        .await?;

    to_result(response).await
}

// TODO: lengthen
pub async fn lengthen(id: String) -> ApiResult<LengthenResponse> {
    let response = reqwest::get(api_url!("/url/{id}/lengthen")).await?;

    // Ok("lengthened".to_string())
    todo!()
}

// TODO: stats
pub async fn stats(id: String) -> ApiResult<StatsResponse> {
    // Ok("stats".to_string())
    todo!()
}
