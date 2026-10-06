use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("unprocessable entity")]
    UnprocessableEntity,

    #[error("invalid request")]
    BadRequest,

    #[error("internal server error")]
    Internal(#[source] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            ApiError::UnprocessableEntity => {
                (StatusCode::UNPROCESSABLE_ENTITY, "UNPROCESSABLE_ENTITY")
            }

            ApiError::BadRequest => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),

            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        let body = Json(ErrorResponse {
            code,
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: &'static str,
    pub message: String,
}
