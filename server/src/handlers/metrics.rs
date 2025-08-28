use axum::{http::StatusCode, response::IntoResponse};
use metrics_exporter_prometheus::PrometheusBuilder;

pub fn init_metrics() {
    // install() returns Result<(), BuildError>, not a handle
    PrometheusBuilder::new()
        .install()
        .expect("Failed to install Prometheus recorder");

    // Initialize some basic metrics
    metrics::describe_counter!("config_requests_total", "Total number of config requests");
    metrics::describe_counter!(
        "config_requests_failed_total",
        "Total number of failed config requests"
    );
    metrics::describe_histogram!(
        "config_request_duration_seconds",
        "Duration of config requests"
    );
}

pub async fn metrics_handler() -> impl IntoResponse {
    // Use the global registry to get metrics
    let registry = prometheus::default_registry();
    let metric_families = registry.gather();

    match prometheus::TextEncoder::new().encode_to_string(&metric_families) {
        Ok(output) => (
            StatusCode::OK,
            [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
            output,
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [("content-type", "text/plain")],
            "Failed to encode metrics".to_string(),
        ),
    }
}

pub async fn prometheus_metrics() -> impl IntoResponse {
    metrics_handler().await
}
