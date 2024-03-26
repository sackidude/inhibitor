mod database;
mod web;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect(".env variable `DATABASE_URL` couldn't be found.");
    let pool = database::get_database_pool(&database_url).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let app = web::get_router(&pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
    pool.close().await; // Probably not necessary, but for clarity. 
}
