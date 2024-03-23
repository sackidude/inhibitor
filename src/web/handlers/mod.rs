use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

pub mod login;
pub mod startpage;

pub mod authorized;

pub fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}
