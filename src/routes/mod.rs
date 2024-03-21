use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

mod handlers;

pub fn get_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(handlers::startpage::get))
        .fallback_service(handlers::routes_static())
        .with_state(pool)
}
