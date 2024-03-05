use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{post::Post, state::AppState};

#[derive(Debug, Deserialize, Default)]
pub struct GetPostsQuery {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn get_posts(
    opts: Option<Query<GetPostsQuery>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.offset.unwrap_or(1) - 1) * limit;

    let res = sqlx::query_as!(
        Post,
        "SELECT * FROM posts ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&state.db)
    .await;

    match res {
        Ok(posts) => Ok((
            StatusCode::OK,
            Json(json!({ "status": "success", "data": posts })),
        )),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Failed to fetch posts".to_string())),
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePostDto {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1, max = 1000))]
    pub body: String,
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreatePostDto>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let res = sqlx::query_as!(
        Post,
        "INSERT INTO posts (title, body) VALUES ($1, $2) RETURNING *",
        body.title.to_string(),
        body.body.to_string()
    )
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(post) => Ok((
            StatusCode::CREATED,
            Json(json!({ "status": "success", "data": post })),
        )),
        Err(_) => return Err((StatusCode::NOT_FOUND, "Failed to create post".to_string())),
    }
}

pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(post) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "data": post
            })),
        )),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            "The requested post could not be found".to_string(),
        )),
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EditPostDto {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 1000))]
    pub body: Option<String>,
}

pub async fn edit_post(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<EditPostDto>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let cur_res = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
        .fetch_one(&state.db)
        .await;

    if cur_res.is_err() {
        return Err((
            StatusCode::NOT_FOUND,
            "The requested post could not be found".to_string(),
        ));
    }

    let now = chrono::Utc::now();
    let post = cur_res.unwrap();

    let res = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = $1, body = $2, updated_at = $3 WHERE id = $4 RETURNING *",
        body.title.to_owned().unwrap_or(post.title),
        body.body.to_owned().unwrap_or(post.body),
        now,
        id
    )
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(post) => Ok((
            StatusCode::OK,
            Json(json!({ "status": "success", "data": post })),
        )),
        Err(_) => return Err((StatusCode::NOT_FOUND, "Failed to update post".to_string())),
    }
}

pub async fn delete_post(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let res = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(&state.db)
        .await;

    if res.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        ));
    }

    match res.unwrap().rows_affected() {
        0 => Err((
            StatusCode::NOT_FOUND,
            format!("Post with ID: {} not found", id),
        )),
        _ => Ok((
            StatusCode::OK,
            Json(json!({ "status": "success", "message": "Post deleted" })),
        )),
    }
}
