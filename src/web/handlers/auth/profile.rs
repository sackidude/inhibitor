use askama::Template;
use axum::{extract, response};

#[derive(askama::Template)]
#[template(path = "profile.html")]
struct ProfileTemplate<'a> {
    username: &'a str,
    password: &'a str
}

pub async fn get(
    extract::State(state): extract::State<sqlx::Pool<sqlx::Postgres>>
) -> impl response::IntoResponse {
    let profile = ProfileTemplate { username: "hello", password: "world" };
    response::Html(profile.render().unwrap())
}