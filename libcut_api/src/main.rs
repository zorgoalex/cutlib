mod concurrency;
mod error_mapping;
mod routes;

use axum::Router;
use axum::routing::{get, post};
use concurrency::ConcurrencyGate;
use routes::AppState;

#[tokio::main]
async fn main() {
    let gate = ConcurrencyGate::from_env();
    let state = AppState { gate };

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/api/cut/optimize", post(routes::optimize))
        .with_state(state);

    let port: u16 = std::env::var("LIBCUT_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);

    let addr = format!("0.0.0.0:{}", port);
    println!("LibCut API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
