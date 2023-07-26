use common::error::UrlError;
use common::types::{
    AllUrlsResponse, ErrorResponse, LengthenResponse, ShortenRequest, ShortenResponse,
    StatsResponse,
};
use reqwest::{Response, Result as ReqResult, StatusCode};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum ApiError {
    Url(StatusCode, UrlError),
    Reqwest(reqwest::Error),
    Other(StatusCode, String),
}
pub type ApiResult<T> = Result<T, ApiError>;

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Url(status, err) => write!(f, "{status}: {err:?}"),
            ApiError::Reqwest(err) => err.fmt(f),
            ApiError::Other(status, body) => write!(f, "{status}: {body}"),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
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
        Err(ApiError::Url(status, res.error))
    } else {
        // server error or network error(?)
        Err(ApiError::Other(status, response.text().await?))
    }
}

/// Basically `format!` but with value of env var `URL_SHORTENER_API_URL` prepended
macro_rules! api_url {
    ($($arg:tt)*) => {{
        // TODO: probably error handle
        // TODO: some sort of option/way to select local or remote - just need to enter URL really,
        // which can be done with env var
        let api_url = std::env::var("URL_SHORTENER_API_URL").unwrap_or("http://localhost:8000/api".to_string());
        // let api_url = "https://url-shortener-server-actix.shuttleapp.rs/api";
        let endpoint = format!($($arg)*);
        format!("{api_url}{endpoint}")
    }}
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

pub async fn lengthen(id: String) -> ApiResult<LengthenResponse> {
    let response = reqwest::get(api_url!("/url/{id}/lengthen")).await?;
    to_result(response).await
}

pub async fn stats(id: String) -> ApiResult<StatsResponse> {
    let response = reqwest::get(api_url!("/url/{id}/stats")).await?;
    to_result(response).await
}
