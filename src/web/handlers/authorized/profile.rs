use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "get_profile.html")]

struct GetProfileTemplate {
    username: String,
    password: String
}

pub async fn get(State(pool): State<PgPool>) -> impl IntoResponse {
    println!("profile get ");
    let profile = GetProfileTemplate { username: "Poop".to_string(), password: "nigga".to_string() };
    Html(profile.render().unwrap())
}