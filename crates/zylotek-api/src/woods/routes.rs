use axum::{Router, routing::get};

use crate::{
    state::AppState,
    woods::handlers::{
        create_wood_handler, delete_wood_handler, get_wood_handler, list_woods_handler,
        update_wood_handler,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_woods_handler).post(create_wood_handler))
        .route(
            "/{id}",
            get(get_wood_handler)
                .put(update_wood_handler)
                .delete(delete_wood_handler),
        )
}
