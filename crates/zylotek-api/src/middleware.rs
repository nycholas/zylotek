use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::Response};

use crate::errors::ApiError;

pub async fn logging_middleware(req: Request, next: Next) -> Result<Response, ApiError> {
    let time = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let response = next.run(req).await;
    log::info!(
        "{} {} -> {} ({:?} ms)",
        method,
        uri,
        response.status(),
        time.elapsed().as_millis()
    );
    Ok(response)
}
