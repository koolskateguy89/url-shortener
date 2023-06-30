use gloo_net::http::Request;

use super::NetResult;
use common::types::{LoginRequest, RegisterRequest};

pub async fn whoami() -> NetResult<String> {
    let response = Request::get("/api/whoami").send().await?;

    response.text().await
}

pub async fn login(username: impl Into<String>, password: impl Into<String>) -> NetResult<bool> {
    let body = LoginRequest {
        username: username.into(),
        password: password.into(),
    };

    let response = Request::post("/api/login").json(&body)?.send().await?;

    Ok(response.ok())
}

pub async fn logout() -> NetResult<bool> {
    let response = Request::post("/api/logout").send().await?;

    Ok(response.ok())
}

pub async fn register(username: impl Into<String>, password: impl Into<String>) -> NetResult<bool> {
    let body = RegisterRequest {
        username: username.into(),
        password: password.into(),
    };

    let response = Request::post("/api/register").json(&body)?.send().await?;

    Ok(response.ok())
}
