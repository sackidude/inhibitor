use std::time::Duration;

use sqlx::Pool;
use sqlx::Postgres;

pub async fn get_database_pool(database_url: &str) -> Pool<Postgres> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
        .expect("Failed to create database pool")
}
