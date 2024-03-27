use askama::Template;
use axum::{extract, response};

#[derive(askama::Template)]
#[template(path = "profile.html")]
struct ProfileTemplate<'a> {
    username: &'a str,
    accounts: Vec<&'a str>,
}

pub async fn get(
    extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,
    cookies: tower_cookies::Cookies,
) -> impl response::IntoResponse {
    let profile = ProfileTemplate {
        username: "hello",
        accounts: vec!["test1", "test2"],
    };
    response::Html(profile.render().unwrap())
}
