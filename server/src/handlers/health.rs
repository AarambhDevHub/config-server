use crate::repository::ConfigRepository;
use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn health_check(
    State(_repository): State<Arc<ConfigRepository>>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "UP",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "checks": {
            "config-server": {
                "status": "UP"
            }
        }
    })))
}

pub async fn liveness() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "UP",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn readiness(
    State(_repository): State<Arc<ConfigRepository>>,
) -> Result<Json<Value>, StatusCode> {
    // Add actual readiness checks here
    Ok(Json(json!({
        "status": "UP",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "checks": {
            "filesystem": {
                "status": "UP"
            }
        }
    })))
}
