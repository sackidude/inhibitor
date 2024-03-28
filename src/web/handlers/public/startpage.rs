use crate::web;
use askama::{self, Template};
use axum::response;

#[derive(askama::Template)]
#[template(path = "startpage.html")]

struct StartPageTemplate {}

pub async fn get(/*extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,*/
) -> web::error::Result<response::Html<String>> {
    let startpage = StartPageTemplate {};
    Ok(response::Html(startpage.render()?))
}
