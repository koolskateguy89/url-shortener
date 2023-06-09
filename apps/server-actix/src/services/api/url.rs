use actix_web::{get, post, web, Responder, Result};
use log::info;

use std::collections::HashMap;

use common::types;

use crate::db;
use crate::{AppState, UserError};

#[get("/api/urls")]
pub async fn get_all_urls(state: web::Data<AppState>) -> Result<impl Responder> {
    let db = sqlx::query_as::<_, (String, String)>("SELECT * FROM urls")
        .fetch_all(&state.pool)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?
        .into_iter()
        .collect::<HashMap<_, _>>();

    Ok(web::Json(db))
}

#[post("/api/url/shorten")]
pub async fn shorten_url(
    body: web::Json<types::ShortenRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    info!("shortening url: {}", body.url);

    let id = db::url::insert_short_url(&state.pool, &body.url).await?;

    Ok(web::Json(types::ShortenResponse { id }))
}

#[get("/api/url/{id}/lengthen")]
pub async fn lengthen_url(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let (id,) = path.into_inner();

    info!("lengthening id: {}", id);

    let url = db::url::get_long_url(&state.pool, &id).await?;

    Ok(web::Json(types::LengthenResponse { url }))
}

#[get("/api/url/{id}/exists")]
pub async fn id_exists(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let (id,) = path.into_inner();

    let exists = db::url::id_exists(&state.pool, &id)
        .await
        .map_err(|err| UserError::Other(err.to_string()))?;

    match exists {
        true => Ok("exists"),
        false => Err(UserError::NotFound.into()),
    }
}

#[get("/api/url/{id}/stats")]
pub async fn lengthen_stats(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let (id,) = path.into_inner();

    let db::url::LengthenStat { url, hits } = db::url::get_lengthen_stats(&state.pool, &id).await?;

    Ok(web::Json(types::StatsResponse {
        url,
        num_hits: hits.len(),
        hits,
    }))
}
