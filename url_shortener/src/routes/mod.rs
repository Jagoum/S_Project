use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use crate::{
    errors::Result, models::{CreateUrl, Url}, services::UrlService
};

pub fn url_routes(service: UrlService) -> Router {
    Router::new()
        .route("/", post(create_short_url))
        .route("/:short_url", get(get_long_url))
        .with_state(service)
}

async fn create_short_url(
    State(service): State<UrlService>,
    Json(payload): Json<CreateUrl>,
) -> Result<Json<Url>> {
    let url = service.create_short_url(payload).await?;
    Ok(Json(url))
}

async fn get_long_url(
    State(service): State<UrlService>,
    Path(short_url): Path<String>,
) -> Result<Json<Url>> {
    let url = service.get_long_url(&short_url).await?;
    Ok(Json(url))
}