use askama::Template;
use axum::{extract, response};

use crate::web::middleware::token;

#[derive(askama::Template)]
#[template(path = "logged_in.html")]
struct LoggedInTemplate {
    login_state: LoginResponse
}
enum LoginResponse {
    Successfull{username: String},
    WrongPassword,
    UserNotFound,
}

#[derive(serde::Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

struct DBResponse {
    id: i32,
    password: String
}

pub async fn post(
    cookies: tower_cookies::Cookies,
    extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,
    req_body: String
) -> impl response::IntoResponse {
    let payload: LoginPayload = serde_urlencoded::from_str(&req_body).unwrap();
    println!("Login request from: {}", payload.username); // TEMP!

    let res = 
        sqlx::query_as!(
            DBResponse,
            "SELECT id, password FROM users WHERE username = ($1) LIMIT 1", 
            payload.username)
        .fetch_optional(&pool)
        .await
        .unwrap();

    match res {
        Some(res) => {
            if res.password == payload.password { // TODO! implement hashing on so forth.
                // Generate auth_token
                let auth_token = token::AuthToken::new(res.id);
                let signature = auth_token.get_signature();

                // Store cookie in db
                // TODO! onload to this to some external function that tracks errors. This solution is temporary.
                let _ = sqlx::query!("UPDATE users SET signature = ($1) WHERE id = ($2)", signature, res.id)
                    .execute(&pool)
                    .await
                    .unwrap();

                // Create and set cookie
                let mut auth_cookie = tower_cookies::Cookie::new(token::AUTH_TOKEN, auth_token.to_str());
                auth_cookie.set_http_only(true);
                auth_cookie.set_path("/");
                auth_cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);
                cookies.add(auth_cookie);

                // Construct response
                get_response(LoginResponse::Successfull { username: payload.username })
            } else {
                get_response(LoginResponse::WrongPassword)
            }
        },
        None => {
            get_response(LoginResponse::UserNotFound)
        },
    }
}

fn get_response(log_response: LoginResponse) -> impl response::IntoResponse{
    let loggedin = LoggedInTemplate{ login_state: log_response };
    response::Html(loggedin.render().unwrap())
}