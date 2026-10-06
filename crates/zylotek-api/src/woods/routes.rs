use axum::{Router, routing::post};

use crate::{state::AppState, woods::handlers::create_wood_handler};

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create_wood_handler))
}
