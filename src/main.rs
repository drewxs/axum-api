use std::{env, sync::Arc};

use anyhow::Result;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use axum_api::{route::create_router, state::AppState};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let origin = env::var("ORIGIN").expect("ORIGIN must be set");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let state = Arc::new(AppState { db: pool.clone() });
    let cors = CorsLayer::new()
        .allow_origin(origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(state).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(cors),
    );

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("{}:{}", host, port);

    println!("listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
