use axum::{extract, response};

pub async fn post(
    extract::State(state): extract::State<sqlx::Pool<sqlx::Postgres>>
) -> impl response::IntoResponse {
    "hello, login post"
}