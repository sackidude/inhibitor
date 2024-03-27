use askama::{self, Template};
use axum::{extract, response};

#[derive(askama::Template)]
#[template(path = "startpage.html")]

struct StartPageTemplate {}

pub async fn get(
    extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,
) -> impl response::IntoResponse {
    let startpage = StartPageTemplate {};
    response::Html(startpage.render().unwrap())
}
