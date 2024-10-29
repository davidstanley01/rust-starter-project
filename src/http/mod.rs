pub mod error;
pub mod models;
pub mod services;
pub mod views;

use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Error;
use axum::extract::Json;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Router};
use axum_prometheus::PrometheusMetricLayer;
use lazy_static::lazy_static;
use serde_json::json;
use tokio::net::TcpListener;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;

use crate::config::AppConfig;
use crate::http::services::Services;
use crate::http::views::{app, health};

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] =
        &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct ApplicationServer;

impl ApplicationServer {
    pub async fn app(config: Arc<AppConfig>) -> Result<Router, Error> {
        let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
        let state = Services::new(config.clone());

        let router = Router::new()
            .nest("/api/v1", app())
            .route("/", get(health))
            .route("/metrics", get(|| async move { metric_handle.render() }))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(Self::handle_timeout_error))
                    .timeout(Duration::from_secs(*HTTP_TIMEOUT))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
            )
            .route_layer(prometheus_layer)
            .fallback(Self::handle_404)
            .with_state(state);

        Ok(router)
    }

    pub async fn serve(config: Arc<AppConfig>) -> anyhow::Result<()> {
        let router = ApplicationServer::app(config.clone()).await?;

        let addr: SocketAddr = (config.host.as_str(), config.port)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Unable to parse listener address"))?;

        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, router.into_make_service())
            .await
            .unwrap();

        Ok(())
    }

    async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
        if err.is::<tower::timeout::error::Elapsed>() {
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error":
                        format!(
                            "request took longer than the configured {} second timeout",
                            *HTTP_TIMEOUT
                        )
                })),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("unhandled internal error: {}", err)
                })),
            )
        }
    }

    async fn handle_404() -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            axum::response::Json(serde_json::json!({
                "errors":{
                    "message": vec!(String::from("The requested resource does not exist on this server!")),
                }
            })),
        )
    }
}
