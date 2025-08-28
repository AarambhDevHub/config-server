use axum::{
    Router,
    routing::{get, post},
};
use config_server::{
    config::ServerConfig,
    handlers::{config, health, metrics},
    repository::ConfigRepository,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "config_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = ServerConfig::from_env()?;
    let repository = Arc::new(ConfigRepository::new(config.clone()).await?);

    // Main application routes
    let app = Router::new()
        .route("/", get(|| async { "Config Server is running!" }))
        .route("/{application}/{profile}/{label}", get(config::get_config))
        .route("/encrypt", post(config::encrypt_value))
        .route("/decrypt", post(config::decrypt_value))
        .route("/refresh", post(config::refresh_configs))
        .layer(CorsLayer::permissive())
        .with_state(repository.clone());

    // Health check routes (separate port)
    let health_app = Router::new()
        .route("/health", get(health::health_check))
        .route("/health/live", get(health::liveness))
        .route("/health/ready", get(health::readiness))
        .with_state(repository.clone());

    // Metrics routes (separate port)
    let metrics_app = Router::new()
        .route("/metrics", get(metrics::metrics_handler))
        .route("/actuator/prometheus", get(metrics::prometheus_metrics));

    // Initialize metrics
    metrics::init_metrics();

    // Start servers
    let main_listener =
        tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", config.server_port)).await?;
    let health_listener =
        tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", config.health_port)).await?;
    let metrics_listener =
        tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", config.metrics_port)).await?;

    tracing::info!("Config Server starting on port {}", config.server_port);
    tracing::info!("Health checks available on port {}", config.health_port);
    tracing::info!("Metrics available on port {}", config.metrics_port);

    // Start all servers concurrently
    tokio::try_join!(
        axum::serve(main_listener, app),
        axum::serve(health_listener, health_app),
        axum::serve(metrics_listener, metrics_app),
    )?;

    Ok(())
}
