use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{state::AppState, user::User, utils::crypto};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    pub email: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateUserDto>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let password = body.password.to_string();
    let hash = crypto::hash(password).await;

    if let Err(e) = hash {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }
    let hash = hash.unwrap();

    let res = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *",
        body.email.to_string(),
        hash
    )
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(user) => Ok((
            StatusCode::CREATED,
            Json(json!({ "status": "success", "data": user })),
        )),
        Err(_) => return Err((StatusCode::NOT_FOUND, "Failed to create user".to_string())),
    }
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(user) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "data": user
            })),
        )),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            "The requested user could not be found".to_string(),
        )),
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginDto>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let email = body.email.to_string();

    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(user) => {
            let password = body.password.to_string();
            let hash = user.password;

            match crypto::verify(password, hash).await {
                Ok(true) => Ok((StatusCode::OK, Json(json!({ "status": "success" })))),
                Ok(false) => Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string())),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            "User with email does not exist".to_string(),
        )),
    }
}
