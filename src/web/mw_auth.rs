use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use sqlx::PgPool;
use tower_cookies::Cookies;

use crate::{error::{Error, Result}, web::{mw_auth::auth_token::AuthToken, AUTH_TOKEN}};

pub mod auth_token;


pub async fn mw_require_auth(
    State(pool): State<PgPool>,
    cookies: Cookies,
    req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("middleware: mw_require_auth");
    let auth_token = AuthToken::from_str(&cookies.get(AUTH_TOKEN).ok_or(Error::NoAuthenticationToken)?.value().to_string())?;

    
    println!("Got token: {:?}", auth_token);

    Ok(next.run(req).await)
}

