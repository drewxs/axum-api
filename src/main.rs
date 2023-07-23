mod db;
mod models;
mod todos;

use crate::{
    db::Db,
    todos::{create_todo, get_todo, get_todos},
};
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = Db::default();

    let todo_routes = Router::new()
        .route("/", get(get_todos).post(create_todo))
        .route("/:id", get(get_todo));
    let api_routes = Router::new().nest("/todos", todo_routes);
    let app = Router::new().nest("/api", api_routes).with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
