use axum::{extract, http, middleware, response::{self, IntoResponse}};

pub mod token;

struct DBSignatureRes { 
    signature: Option<String> 
}

pub async fn require_auth(
    extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,
    cookies: tower_cookies::Cookies,
    req: http::Request<axum::body::Body>,
    next: middleware::Next
) -> response::Response {
    let auth_cookie = match cookies.get(token::AUTH_TOKEN) {
        Some(cookie) => cookie,
        None => return auth_failed_response(),
    };


    let auth_token = match token::AuthToken::from_str(&auth_cookie.value().to_string()) {
        Some(token) => token,
        None => return auth_failed_response(),
    };

    if auth_token.is_expired() {
        return (http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let signature = match sqlx::query_as!(DBSignatureRes, "SELECT signature FROM users WHERE id = ($1)", auth_token.get_user_id())
        .fetch_optional(&pool)
        .await
        .unwrap() {
            Some(res) => match res.signature {
                Some(signature) => signature,
                None => return auth_failed_response(),
            },
            None => return auth_failed_response(),
        };

    if auth_token.get_signature() != signature {
        return auth_failed_response();
    }

    next.run(req).await
}

fn auth_failed_response() -> response::Response {
    return (http::StatusCode::UNAUTHORIZED, "No authorization, try login again").into_response();
}