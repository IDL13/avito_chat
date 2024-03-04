use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use serde::{Serialize, Deserialize};

pub enum ApiResponse {
    JsonDataStr(&'static str),
    JsonDataI32(i32),
    JsonChats(Vec<ChatResp>),
    JsonMessages(Vec<MessageResp>)
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::JsonDataStr(data) => (StatusCode::OK, Json(json!({"msg":data}))).into_response(),
            Self::JsonDataI32(data) => (StatusCode::OK, Json(json!({"msg":data}))).into_response(),
            Self::JsonChats(data) => (StatusCode::OK, Json(json!({"msg":data}))).into_response(),
            Self::JsonMessages(data) => (StatusCode::OK, Json(json!({"msg":data}))).into_response(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserGet {
    pub user: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub id: Option<i32>,
    pub name: String,
    pub users: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatGet {
    pub chat: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResp {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Option<i32>,
    pub chat: Option<i32>,
    pub author: Option<i32>,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageResp {
    pub text: String,
}

