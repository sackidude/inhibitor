use axum::{extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn get(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    Ok(String::from("Hello, World!"))
}
