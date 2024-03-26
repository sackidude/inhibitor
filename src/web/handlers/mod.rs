use axum::routing;
use tower_http::services;

pub mod public;
pub mod auth;

pub fn static_routes() -> axum::Router {
    axum::Router::new().nest_service("/", routing::get_service(services::ServeDir::new("./static")))
}
