use axum::{routing::{get, post}, Router};

mod handlers;
mod postgres_db;

use handlers::{test, add_new_user, chats_add, messages_add, chats_get, messages_get};

#[tokio::main]
async fn main() {
    handlers::scheema_db().await.expect("Error from db connection");

    let app = Router::new()
        .route("/", get(test))
        .route("/users/add", post(add_new_user))
        .route("/chats/add", post(chats_add))
        .route("/messages/add", post(messages_add))
        .route("/chats/get", post(chats_get))
        .route("/messages/get", post(messages_get));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    println!("[SERVER START] on 9000 port");
    axum::serve(listener, app).await.unwrap();
}
