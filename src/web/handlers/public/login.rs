use askama::Template;
use axum::{extract, response};

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

pub async fn post(
    extract::State(state): extract::State<sqlx::Pool<sqlx::Postgres>>
) -> impl response::IntoResponse {
    let loggedin = LoggedInTemplate { login_state: LoginResponse::Successfull { username: "hello".to_string() } };
    response::Html(loggedin.render().unwrap())
}