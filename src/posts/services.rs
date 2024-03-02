use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::{
    postgres::{PgQueryResult, PgRow},
    Pool, Postgres,
};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn get_todos(
    pagination: Option<Query<Pagination>>,
    State(db): State<Pool<Postgres>>,
) -> Result<Json<Vec<PgRow>>> {
    let Query(_) = pagination.unwrap_or_default();

    let todos = sqlx::query("SELECT * FROM posts ORDER BY created_at DESC")
        .fetch_all(&db)
        .await?;

    Ok(Json(todos))
}

pub async fn get_todo(
    Path(id): Path<Uuid>,
    State(db): State<Pool<Postgres>>,
) -> Result<Json<PgQueryResult>> {
    let post = sqlx::query("SELECT * FROM posts WHERE id = ?")
        .bind(id)
        .execute(&db)
        .await?;

    Ok(Json(post))
}

#[derive(Deserialize, Validate)]
pub struct CreatePostRequest {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1, max = 1000))]
    pub body: String,
}

pub async fn create_todo(
    State(db): State<Pool<Postgres>>,
    Json(input): Json<CreatePostRequest>,
) -> Result<String> {
    input.validate()?;

    sqlx::query(
        "INSERT INTO posts (title, body, created_at, updated_at) VALUES (?, ?, NOW(), NOW()))",
    )
    .bind(input.title)
    .bind(input.body)
    .execute(&db)
    .await?;

    Ok("Post created".to_string())
}
