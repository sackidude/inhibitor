use askama::Template;
use axum::response::Html;
use axum::{
    extract::State,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_urlencoded;
use sqlx::PgPool;
use sqlx::{self, prelude::FromRow};
use tower_cookies::cookie::{self, SameSite};
use tower_cookies::{Cookie, Cookies};
use crate::{error, web};
use web::mw_auth::auth_token::AuthToken;
use crate::web::{ AUTH_TOKEN};

enum LoginResponse<'a> {
    Successfull { username: &'a str },
    WrongPassword,
    UserNotFound,
}

#[derive(Template)]
#[template(path = "logged_in.html")]
struct LoggedInTemplate<'a> {
    login_state: LoginResponse<'a>,
}


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(FromRow)]
struct DatabaseResponse {
    id: i32,
    password: String,
}

// TODO!: input validation, prevent sql injection.
pub async fn post(
    cookies: Cookies,
    State(pool): State<PgPool>, 
    payload: String
) -> error::Result<impl IntoResponse> {
    let payload = serde_urlencoded::from_str::<LoginPayload>(&payload)
        .map_err(|_|error::Error::RequestParsingError)?;
    
    let res = sqlx::query_as!(
        DatabaseResponse,
        "SELECT id, password FROM users WHERE username = ($1) LIMIT 1",
        payload.username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_|error::Error::DatabaseQueryError)?;

    let logged_in = match res {
        None => {
            LoggedInTemplate{login_state: LoginResponse::UserNotFound}
        }
        Some(res) => {
            if res.password == payload.password {
                let auth_token = AuthToken::new(res.id);
                let signature = auth_token.get_signature();

                // Update database
                let _ = sqlx::query!("UPDATE users SET signature=($1) WHERE id=($2)", signature, res.id)
                    .execute(&pool)
                    .await
                    .map_err(|_|error::Error::DatabaseQueryError)?;

                let mut cookie = Cookie::new(AUTH_TOKEN, auth_token.to_str());
                cookie.set_http_only(true);
                cookie.set_path("/");
                cookie.set_same_site(cookie::SameSite::Strict);
                cookies.add(cookie);
                LoggedInTemplate { login_state: LoginResponse::Successfull { username: &payload.username }}
            } else {
                LoggedInTemplate { login_state: LoginResponse::WrongPassword }
            }
        }
    };
    Ok(Html(logged_in.render().map_err(|_|error::Error::AskamaTemplatingError)?))
}
