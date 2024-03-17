use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::{
    post::{create_post, delete_post, edit_post, get_post, get_posts},
    state::AppState,
    user::{create_user, get_user},
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthcheck", get(|| async { "OK" }))
        .route("/api/v1/post", get(get_posts).post(create_post))
        .route(
            "/api/v1/post/:id",
            get(get_post).patch(edit_post).delete(delete_post),
        )
        .route("/api/v1/user", post(create_user))
        .route("/api/v1/user/:id", get(get_user))
        .with_state(state)
}
