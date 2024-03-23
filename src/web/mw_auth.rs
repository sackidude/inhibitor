use axum::{body::Body, http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{error::{Error, Result}, web::AUTH_TOKEN};

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("middleware: mw_require_auth");
    let auth_token = match cookies.get(AUTH_TOKEN) {
        Some(cookie) => cookie.value().to_string(),
        None => String::from("Poop"),
    };

    println!("Token gotten: {}", auth_token);


    Ok(next.run(req).await)
}

