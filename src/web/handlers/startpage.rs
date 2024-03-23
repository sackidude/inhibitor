use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "startpage.html")]

struct StartpageTemplate {}

pub async fn get(State(pool): State<PgPool>) -> impl IntoResponse {
    let startpage = StartpageTemplate {};
    Html(startpage.render().unwrap())
}
