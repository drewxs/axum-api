mod models;
mod state;
mod todos;

use std::env;

use crate::{
    state::Cache,
    todos::{create_todo, get_todo, get_todos},
};
use axum::{routing::get, Router};
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cache = Cache::default();

    let todo_routes = Router::new()
        .route("/", get(get_todos).post(create_todo))
        .route("/:id", get(get_todo));

    let v1_routes = Router::new().nest("/todos", todo_routes);

    let app = Router::new()
        .route("/healthcheck", get(|| async { "OK" }))
        .nest("/api/v1", v1_routes)
        .with_state(cache);

    let host = env::var("HOST").expect("HOST must be set in .env");
    let port = env::var("PORT").expect("PORT must be set in .env");
    let addr = format!("{}:{}", host, port);

    tracing::debug!("listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
