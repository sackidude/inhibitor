use askama::Template;
use axum::{extract, response};
use sqlx::prelude::FromRow;

use crate::web::middleware;

#[derive(FromRow)]
struct Account {
    name: String,
    id: i32,
}

#[derive(askama::Template)]
#[template(path = "profile.html")]
struct ProfileTemplate {
    accounts: Vec<Account>,
}

pub async fn get(
    extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,
    cookies: tower_cookies::Cookies,
) -> impl response::IntoResponse {
    // parse token
    let token = match middleware::get_auth_token(cookies) {
        Some(token) => token,
        None => panic!("Request should have valid auth token after middleware"), // TODO FIX THIS
    };

    let user_id = token.get_user_id();

    // unchecked because it thinks the concat returns optional string.
    let accounts = sqlx::query_as_unchecked!(
        Account,
        r#"SELECT
        CONCAT(account.in_game_name, '#', account.tag) "name",
        account.id
    FROM
        account
        INNER JOIN user_account ON account.id = user_account.account_id
    WHERE
        user_account.user_id = ($1)"#,
        user_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let profile = ProfileTemplate { accounts };
    response::Html(profile.render().unwrap())
}
