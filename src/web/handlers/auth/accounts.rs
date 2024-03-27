use axum::{extract, response};

use crate::{api, web::middleware};

struct AccountId {
    id: i32,
}

pub async fn post(
    cookies: tower_cookies::Cookies,
    extract::State(pool): extract::State<sqlx::Pool<sqlx::Postgres>>,
    req_body: String,
) -> impl response::IntoResponse {
    // Get account
    let account = api::Account::from_form_res(&req_body).await.unwrap();

    // Parse auth token to get user id
    let token = match middleware::get_auth_token(cookies) {
        Some(token) => token,
        None => panic!("Request should have valid auth token after middleware"), // TODO FIX THIS
    };

    let user_id = token.get_user_id();

    // Push the account to the database
    let account_id = sqlx::query_as_unchecked!(
        AccountId,
        r#"INSERT INTO
    account (in_game_name, region, tag)
VALUES
    ($1, CAST ($2 AS REGION), $3) 
RETURNING id;"#,
        account.get_username(),
        account.get_region(),
        account.get_tag()
    )
    .fetch_one(&pool)
    .await
    .unwrap();

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
    .await
    .unwrap();

    "added"
}
