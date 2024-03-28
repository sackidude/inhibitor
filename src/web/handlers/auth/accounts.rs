pub mod games;

use axum::{extract, response};

use crate::{
    api,
    web::{self, middleware},
};

struct AccountId {
    id: i32,
}

pub async fn post(
    cookies: tower_cookies::Cookies,
    extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,
    req_body: String,
) -> web::error::Result<axum::http::StatusCode> {
    // Get account
    let account = api::Account::from_form_res(&req_body).await?;

    // Parse auth token to get user id
    let token = middleware::get_auth_token(cookies).ok_or(()).expect("profile::get couldn't get auth token from response when middleware should garante there to be one");

    let user_id = token.get_user_id();

    // TODO! Verify that the account does not already exist in the database.
    // Push the account to the database
    let account_id = sqlx::query_as_unchecked!(
        AccountId,
        r#"INSERT INTO
    account (in_game_name, region, tag)
VALUES
    ($1, CAST ($2 AS REGION), $3) 
RETURNING id;"#,
        account.get_username(),
        Into::<&str>::into(account.get_region()),
        account.get_tag()
    )
    .fetch_one(&pool)
    .await?;

    // Add entry to user_account
    sqlx::query!(
        "INSERT INTO
    user_account (user_id, account_id)
VALUES
    ($1, $2)",
        user_id,
        account_id.id
    )
    .execute(&pool)
    .await?;

    Ok(axum::http::StatusCode::CREATED)
}
