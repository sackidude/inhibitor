use axum::routing;

mod handlers;
mod middleware;

pub fn get_router(pool: &sqlx::Pool<sqlx::Postgres>) -> axum::Router {
    let public_routes = axum::Router::new()
        .route("/", routing::get(handlers::public::startpage::get))
        .route("/login", routing::post(handlers::public::login::post));

    let auth_routes = axum::Router::new()
        .route("/profile", routing::get(handlers::auth::profile::get))
        .route_layer(axum::middleware::from_fn_with_state(pool.clone(), middleware::require_auth))
        .layer(tower_cookies::CookieManagerLayer::new());

    axum::Router::new()
        .route("/", routing::get(handlers::public::startpage::get))
        .nest("/public", public_routes)
        .nest("/auth", auth_routes)
        .with_state(pool.clone())
        .fallback_service(handlers::static_routes())
}