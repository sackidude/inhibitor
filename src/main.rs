use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;
use std::error::Error;
use std::time::Duration;

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(handlers::startpage::get))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
