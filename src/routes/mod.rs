use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

mod handlers;

pub fn get_router(pool: Pool<Postgres>) -> Router {
    use handlers as h;
    Router::new()
        .route("/", get(h::startpage::get))
        .route("/login", post(h::login::post))
        .fallback_service(h::routes_static())
        .with_state(pool)
}
