use sqlx::{postgres::PgPoolOptions, PgPool};
use env_file_reader::read_file;

pub async fn connection()-> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(read_file("./.env").expect("Error from read .env")["DB_URL"].as_str()).await.expect("Error from db connection")
}

