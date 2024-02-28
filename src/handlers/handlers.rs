use axum::{
    http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use serde_json::{Value, json};

use super::{models::{ApiResponse, User}};

pub async fn test() -> ApiResponse {
    ApiResponse::JsonDataStr("[SERVER START]")
}

pub async fn add_new_user(Json(json): Json<User>) -> ApiResponse {
    let pool = crate::postgres_db::connection().await;

    let row: (i32,) = sqlx::query_as("INSERT INTO user (username, created_at) VALUES ($1, $2) RETURNING id")
    .bind(json.username)
    .bind(json.created_at)
    .fetch_one(&pool)
    .await
    .expect("Error from add user in db");

    ApiResponse::JsonDataI32(row.0)
}