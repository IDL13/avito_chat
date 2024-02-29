use axum::{routing::{get, post}, Router};

mod handlers;
mod postgres_db;

#[tokio::main]
async fn main() {
    handlers::scheema_db().await.expect("Error from db connection");

    let app = Router::new()
        .route("/", get(handlers::test))
        .route("/users/add", post(handlers::add_new_user))
        .route("/chats/add", post(handlers::chats_add))
        .route("/messages/add", post(handlers::messages_add));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    println!("[SERVER START] on 9000 port");
    axum::serve(listener, app).await.unwrap();
}
