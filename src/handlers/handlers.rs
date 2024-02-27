use axum::{
    http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use serde_json::{Value, json};

pub enum ApiResponse {
    OK,
    Create,
    JsonData(&'static str),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Create => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(json!({"msg":data}))).into_response(),
        }
    }
}

pub async fn test() -> ApiResponse {
    ApiResponse::JsonData("[SERVER START]")
}