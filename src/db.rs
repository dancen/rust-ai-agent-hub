use sqlx::mysql::MySqlPoolOptions;
use dotenvy::dotenv;
use std::env;

pub async fn init_db() -> sqlx::MySqlPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}