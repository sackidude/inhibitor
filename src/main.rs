use database::get_database_pool;
use routes::get_router;

mod database;
mod error;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect(".env variable `DATABASE_URL` couldn't be found.");
    let pool = get_database_pool(&database_url).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let app = get_router(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
