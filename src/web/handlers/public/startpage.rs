use axum::{extract, response};

pub async fn get(
    extract::State(state): extract::State<sqlx::Pool<sqlx::Postgres>>
) -> impl response::IntoResponse {
    "hello, startpage"
}