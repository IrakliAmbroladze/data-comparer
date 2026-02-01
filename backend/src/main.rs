use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

mod comparison;
mod handlers;
mod parsers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/upload", post(handlers::upload_handler))
        .route("/compare", post(handlers::compare_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Backend server listening on {}", addr);

    let listener: TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Data Comparer Backend API"
}

async fn health_check() -> &'static str {
    "OK"
}
