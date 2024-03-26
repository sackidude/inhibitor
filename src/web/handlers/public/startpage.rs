use axum::{extract, response};
use askama::{self, Template};

#[derive(askama::Template)]
#[template(path = "startpage.html")]

struct StartPageTemplate {}

pub async fn get(
    extract::State(_state): extract::State<sqlx::Pool<sqlx::Postgres>>
) -> impl response::IntoResponse {
    let startpage = StartPageTemplate{};
    response::Html(startpage.render().unwrap())
}