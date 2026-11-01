use std::time::Duration;

use axum::Router;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{state::AppState, woods};

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api/v1/woods", woods::routes::router())
        .with_state(state)
        .fallback(not_found_handler)
        .layer(axum::middleware::from_fn(
            crate::middleware::logging_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(5),
        ))
        .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1MB
        .layer(cors)
}

async fn not_found_handler() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "404 Not Found")
}
