use axum::Router;

use crate::{state::AppState, woods};

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/woods", woods::routes::router())
        .with_state(state)
}
