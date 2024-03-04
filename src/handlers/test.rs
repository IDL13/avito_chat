#[cfg(test)]
use axum::http::StatusCode;
use ::axum::Router;
use ::axum::{routing::{get, post}, body::Body};
use ::axum_test::TestServer;
use serde_json::json;
use crate::handlers::models::ApiResponse;
use serde::{Serialize, Deserialize};

#[tokio::test]
async fn it_should_get() {
    let app = Router::new()
        .route("/", get(crate::handlers::test));

    let server = TestServer::new(app).unwrap();


    let response = server
        .get("/")
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body = response.as_bytes();
    let json_content: serde_json::Value = serde_json::from_slice(&body).expect("Error");
    assert_eq!(json_content, json!({"msg":"[SERVER START]"}));
}

#[tokio::test]
async fn test_add_new_user() {
    crate::handlers::scheema_db().await.expect("Error from db connection");
    
    let app = Router::new()
        .route("/users/add", post(crate::handlers::add_new_user));

    let server = TestServer::new(app).unwrap();

    let test_str = r#"{"username":"test"}"#;
    let json: serde_json::Value = serde_json::from_str(&test_str).expect("Error");

    let response = server
        .post("/users/add")
        .json(&json)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body = response.as_bytes();
    let json_content: serde_json::Value = serde_json::from_slice(&body).expect("Error");

    assert_eq!(json_content, json!({"msg":2}));

}


