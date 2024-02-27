use axum::{
    http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use serde_json::{Value, json};

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::test));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
