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

enum LoginResponse {
    Successfull { username: String },
    WrongPassword,
    UserNotFound
}

#[derive(Template)]
#[template(path = "logged_in.html")]
struct LoggedInTemplate {
    login_state: LoginResponse,
}


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(FromRow)]
struct DatabaseResponse {
    password: String,
}

pub async fn post(State(pool): State<PgPool>, payload: String) -> impl IntoResponse {
    let req = serde_urlencoded::from_str::<LoginPayload>(&payload).unwrap();
    let res = sqlx::query_as::<_, DatabaseResponse>(
        "SELECT password FROM users WHERE username = ($1) LIMIT 1",
    )
    .bind(&req.username)
    .fetch_optional(&pool)
    .await
    .unwrap();
    let logged_in = match res {
        None => {
            LoggedInTemplate{login_state: LoginResponse::UserNotFound}
        }
        Some(res) => {
            if res.password == req.password {
                LoggedInTemplate { login_state: LoginResponse::Successfull { username: req.username }}
            } else {
                LoggedInTemplate { login_state: LoginResponse::WrongPassword }
            }
        }
    };
    Html(logged_in.render().unwrap())
}
