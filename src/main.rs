use std::env;

use anyhow::Result;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use axum_api::posts::{create_todo, get_todo, get_todos};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&db).await?;

    let todo_routes = Router::new()
        .route("/", get(get_todos).post(create_todo))
        .route("/:id", get(get_todo));

    let v1_routes = Router::new().nest("/todos", todo_routes);

    let app = Router::new()
        .route("/healthcheck", get(|| async { "OK" }))
        .nest("/api/v1", v1_routes)
        .with_state(db);

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("{}:{}", host, port);

    println!("listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
