use std::sync::Arc;


use axum::routing::get;
use axum::routing::post;
use axum::Router;
use rust_prototype::handlers::hello::*;
use rust_prototype::handlers::another_page::*;
use rust_prototype::handlers::blog_list::*;
use rust_prototype::handlers::todo::{AppState, add_todo};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use diesel::prelude::*;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_prototype=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");



    //let router = Router::new().route("/", get(hello));
    let app_state = Arc::new(AppState {
        todos: Mutex::new(vec![]),
    });

    let assets_path = std::env::current_dir().unwrap();
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let api_router = Router::new()
        .route("/hello", get(hello_from_the_server))
        .route("/todos", post(add_todo))
        .with_state(app_state);


    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(hello))
        .route("/another-page", get(another_page))
        .route("/blog_list", get(blog_list))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );


    info!("router initialized, now listening on port {}", port);

    // router initialized, now listening on port 8000
    axum_server::bind(addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}