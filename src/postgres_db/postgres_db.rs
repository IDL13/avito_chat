use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connection()-> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test").await.expect("Error from db connection")
}

