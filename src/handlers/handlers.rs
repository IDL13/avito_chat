use axum::Json;

use super::models::{ApiResponse, Chat, Message, User};

pub async fn test() -> ApiResponse {
    ApiResponse::JsonDataStr("[SERVER START]")
}

pub async fn add_new_user(Json(json): Json<User>) -> ApiResponse {
    let pool = crate::postgres_db::connection().await;

    let row: (i32,) = sqlx::query_as("INSERT INTO Users (username) VALUES ($1) RETURNING id")
    .bind(json.username)
    .fetch_one(&pool)
    .await
    .expect("Error from add user in db");

    ApiResponse::JsonDataI32(row.0)
}

pub async fn chats_add(Json(json): Json<Chat>) -> ApiResponse {
    let pool = crate::postgres_db::connection().await;

    let row: (i32,) = sqlx::query_as("INSERT INTO Chat (name, users) VALUES ($1, $2) RETURNING id")
    .bind(json.name)
    .bind(json.users)
    .fetch_one(&pool)
    .await
    .expect("Error from add chat in db");

    ApiResponse::JsonDataI32(row.0)
}

pub async fn messages_add(Json(json): Json<Message>) -> ApiResponse {
    let pool = crate::postgres_db::connection().await;

    let row: (i32,) = sqlx::query_as("INSERT INTO Message (chat, author, text) VALUES ($1, $2, $3) RETURNING id")
    .bind(json.chat)
    .bind(json.author)
    .bind(json.text)
    .fetch_one(&pool)
    .await
    .expect("Error from add message in db");

    ApiResponse::JsonDataI32(row.0)
}

pub async fn scheema_db() -> Result<(), sqlx::Error> {
    let pool = crate::postgres_db::connection().await;

    sqlx::query("CREATE TABLE IF NOT EXISTS Users (
        id SERIAL PRIMARY KEY,
        username VARCHAR(512) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW())")
    .execute(&pool)
    .await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS Chat (
        id SERIAL PRIMARY KEY,
        name VARCHAR(512) UNIQUE NOT NULL,
        users INTEGER[],
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW())")
    .execute(&pool)
    .await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS Message (id SERIAL PRIMARY KEY,
        chat INTEGER REFERENCES Chat,
        author INTEGER REFERENCES Users,
        text VARCHAR(512),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW())")
    .execute(&pool)
    .await?;

    Ok(())
}