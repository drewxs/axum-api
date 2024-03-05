use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    post::{create_post, delete_post, edit_post, get_post, get_posts},
    state::AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthcheck", get(|| async { "OK" }))
        .route("/api/v1/post", get(get_posts).post(create_post))
        .route(
            "/api/v1/post/:id",
            get(get_post).patch(edit_post).delete(delete_post),
        )
        .with_state(state)
}
