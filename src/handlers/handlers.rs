use axum::{
    http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use serde_json::{Value, json};

use super::{models::{ApiResponse, User}};

pub async fn test() -> ApiResponse {
    ApiResponse::JsonDataStr("[SERVER START]")
}

pub async fn add_new_user(Json(json): Json<User>) -> ApiResponse {
    let pool = crate::postgres_db::connection().await;

    let row: (i32,) = sqlx::query_as("INSERT INTO users (username, created_at) VALUES ($1, $2) RETURNING id")
    .bind(json.username)
    .bind(json.created_at)
    .fetch_one(&pool)
    .await
    .expect("Error from add user in db");

    ApiResponse::JsonDataI32(row.0)
}

pub async fn scheema_db() -> Result<(), sqlx::Error> {
    let pool = crate::postgres_db::connection().await;

    sqlx::query("CREATE TABLE Users (id SERIAL PRIMARY KEY, username VARCHAR(512) NOT NULL, created_at VARCHAR(512))")
    .execute(&pool)
    .await?;

    sqlx::query("CREATE TABLE Chat (id SERIAL PRIMARY KEY, name VARCHAR(512) UNIQUE NOT NULL, users INTEGER[] REFERENCES Users, created_at VARCHAR(512))")
    .execute(&pool)
    .await?;

    sqlx::query("CREATE TABLE Message (id SERIAL PRIMARY KEY, chat INTEGER REFERENCES Chat, author INTEGER REFERENCES Users, text VARCHAR(512), created_at VARCHAR(512))")
    .execute(&pool)
    .await?;

    Ok(())
}