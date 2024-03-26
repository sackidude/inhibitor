pub async fn get_database_pool(database_url: &str) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .connect(database_url)
        .await
}