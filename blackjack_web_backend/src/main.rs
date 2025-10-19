use axum::{
    Router, http,
    routing::{get, post},
};
use http::method::Method;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

mod handler;
mod model;

#[tokio::main]
async fn main() {
    let cors_layer = CorsLayer::new() // 允许所有来源，可换成具体域
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .route("/api", get(health_check))
        .route("/api/game/new", post(handler::new_game))
        // 添加 CORS 层
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Server is running on port 8080");

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "BlackJack Server is running!"
}
