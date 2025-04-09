use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".to_string(),
    })
}

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}
