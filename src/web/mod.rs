use axum::routing;
use handlers::{auth, public};

mod error;
mod handlers;
mod middleware;

pub fn get_router(pool: &sqlx::Pool<sqlx::Postgres>) -> axum::Router {
    let public_routes = axum::Router::new()
        .route("/", routing::get(public::startpage::get))
        .route("/login", routing::post(public::login::post));

    let auth_routes = axum::Router::new()
        .route("/profile", routing::get(auth::profile::get))
        .route("/accounts", routing::post(auth::accounts::post))
        .route("/accounts/games", routing::get(auth::accounts::games::get))
        .route_layer(axum::middleware::from_fn_with_state(
            pool.clone(),
            middleware::require_auth,
        ));

    axum::Router::new()
        .route("/", routing::get(public::startpage::get))
        .nest("/public", public_routes)
        .nest("/auth", auth_routes)
        .layer(tower_cookies::CookieManagerLayer::new())
        .with_state(pool.clone())
        .fallback_service(handlers::static_routes())
}
