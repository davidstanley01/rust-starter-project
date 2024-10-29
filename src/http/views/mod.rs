use axum::extract::{Json, State};
use axum::routing::{get, post};
use axum::Router;
use axum_valid::Valid;
use tracing::info;

use crate::http::error::AppResult;
use crate::http::models::{HealthCheckResponse, ValidationRequest, ValidationResponse};
use crate::http::services::Services;

pub fn app() -> Router<Services> {
    Router::new()
        // /api/v1/
        .route("/", get(home))
        // /api/v1/validation
        .route("/validation", post(validation))
}

pub async fn health() -> AppResult<Json<HealthCheckResponse>> {
    info!("recieved home request");

    Ok(Json(HealthCheckResponse::default()))
}

pub async fn home() -> AppResult<Json<HealthCheckResponse>> {
    info!("recieved home request");

    Ok(Json(HealthCheckResponse::default()))
}

pub async fn validation(
    State(services): State<Services>,
    request: Valid<Json<ValidationRequest>>,
) -> AppResult<Json<ValidationResponse>> {
    info!(
        "recieved request to validate message {:?}",
        request.message.as_ref().unwrap()
    );

    let _ = services.config;

    Ok(Json(ValidationResponse {
        message: "Ok".to_string(),
    }))
}
