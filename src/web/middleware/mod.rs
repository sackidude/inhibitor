use axum::{extract, http, middleware, response};

pub async fn require_auth(
    extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,
    cookies: tower_cookies::Cookies,
    req: http::Request<axum::body::Body>,
    next: middleware::Next
) -> response::Response {
    next.run(req).await
}