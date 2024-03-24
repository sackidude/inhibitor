use axum::{
    middleware, routing::{get, post}, Router
};
use sqlx::{Pool, Postgres};
use tower_cookies::{CookieManager, CookieManagerLayer};

use crate::web::mw_auth::mw_require_auth;

mod handlers;
mod mw_auth;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn get_router(pool: Pool<Postgres>) -> Router {
    use handlers as h;
    
    let auth_routes = Router::new()
        .route("/profile", get(h::authorized::profile::get))
        .route_layer(middleware::from_fn(mw_require_auth))
        .with_state(pool);

    Router::new()
        .route("/", get(h::startpage::get))
        .route("/login", post(h::login::post))
        .nest("/authorized", auth_routes)
        .with_state(pool)
        .layer(CookieManagerLayer::new())
        .fallback_service(h::routes_static())
}
