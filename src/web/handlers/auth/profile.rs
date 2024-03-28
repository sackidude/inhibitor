use askama::Template;
use axum::{extract, response};
use sqlx::prelude::FromRow;

use crate::web::{self, middleware};

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
) -> web::error::Result<response::Html<String>> {
    // parse token
    let token = middleware::get_auth_token(cookies).ok_or(()).expect("profile::get couldn't get auth token from response when middleware should garante there to be one");

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
    .await?;

    let profile = ProfileTemplate { accounts };
    Ok(response::Html(profile.render()?))
}
