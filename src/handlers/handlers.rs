use futures::TryStreamExt;
use sqlx::Row;
use axum::Json;
use super::models::{ApiResponse, Chat, ChatResp, Message, User, UserGet, MessageResp, ChatGet};


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

pub async fn chats_get(Json(json): Json<UserGet>) -> ApiResponse {
    let pool = crate::postgres_db::connection().await;

    let mut response: Vec<ChatResp> = Vec::new();

    let mut rows = sqlx::query("SELECT * FROM Chat WHERE $1 = ANY(users)")
        .bind(json.user)
        .fetch(&pool);

    while let Some(row) = rows.try_next().await.expect("Error from Chat execution") {
        let id: i32 = row.try_get("id").expect("Query dont have id");
        let name: String = row.try_get("name").expect("Query dont have name");

        let json_str = format!(r#"{{ "id":{}, "name":"{}"}}"#, id, name).to_string();
        let json: ChatResp = serde_json::from_str(&json_str).expect("Error from json serialize");

        response.push(json);
    }

    ApiResponse::JsonChats(response)
}

pub async fn messages_get(Json(json): Json<ChatGet>) -> ApiResponse {
    let pool = crate::postgres_db::connection().await;

    let mut response: Vec<MessageResp> = Vec::new();

    let mut rows = sqlx::query("SELECT * FROM Message WHERE chat = $1")
        .bind(json.chat)
        .fetch(&pool);

    while let Some(row) = rows.try_next().await.expect("Error from Chat execution") {
        let text: String = row.try_get("text").expect("Query dont have text");

        let json_str = format!(r#"{{ "text":"{}" }}"#, text).to_string();
        let json: MessageResp = serde_json::from_str(&json_str).expect("Error from json serialize");

        response.push(json);
    }

    ApiResponse::JsonMessages(response)
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

    sqlx::query("CREATE TABLE IF NOT EXISTS Message (
        id SERIAL PRIMARY KEY,
        chat INTEGER REFERENCES Chat,
        author INTEGER REFERENCES Users,
        text VARCHAR(512),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW())")
    .execute(&pool)
    .await?;

    Ok(())
}