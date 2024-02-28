use axum::{
    http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};

pub enum ApiResponse {
    OK,
    Create,
    JsonDataStr(&'static str),
    JsonDataI32(i32)
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Create => (StatusCode::CREATED).into_response(),
            Self::JsonDataStr(data) => (StatusCode::OK, Json(json!({"msg":data}))).into_response(),
            Self::JsonDataI32(data) => (StatusCode::OK, Json(json!({"msg":data}))).into_response()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub id: Option<i32>,
    pub name: String,
    pub users: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Option<i32>,
    pub chat: String,
    pub author: String,
    pub text: String,
    pub created_at: String,
}
