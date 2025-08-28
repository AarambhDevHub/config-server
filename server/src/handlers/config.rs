use crate::{models::*, repository::ConfigRepository, utils::encryption};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;

pub async fn get_config(
    Path((application, profile, label)): Path<(String, String, String)>,
    State(repository): State<Arc<ConfigRepository>>,
) -> Result<Json<ConfigResponse>, StatusCode> {
    match repository.get_config(&application, &profile, &label).await {
        Ok(config) => {
            // Increment metrics
            metrics::counter!("config_requests_total", "application" => application.clone(), "profile" => profile.clone()).increment(1);
            Ok(Json(config))
        }
        Err(_) => {
            metrics::counter!("config_requests_failed_total", "application" => application, "profile" => profile).increment(1);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn encrypt_value(
    Json(request): Json<EncryptRequest>,
) -> Result<Json<EncryptResponse>, StatusCode> {
    match encryption::encrypt(&request.value) {
        Ok(encrypted) => Ok(Json(EncryptResponse { encrypted })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn decrypt_value(
    Json(request): Json<DecryptRequest>,
) -> Result<Json<DecryptResponse>, StatusCode> {
    match encryption::decrypt(&request.encrypted) {
        Ok(decrypted) => Ok(Json(DecryptResponse { decrypted })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn refresh_configs(
    State(repository): State<Arc<ConfigRepository>>,
) -> Result<&'static str, StatusCode> {
    match repository.refresh().await {
        Ok(_) => Ok("Configurations refreshed successfully"),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
