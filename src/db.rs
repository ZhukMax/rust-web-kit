use sea_orm::{Database, DatabaseConnection};
use std::env;

pub fn get_database_url() -> String {
    let user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let port = env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string());
    let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "sea_ms_db".to_string());

    format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, database_name
    )
}

pub async fn get_connection() -> DatabaseConnection {
    let database_url = get_database_url();
    Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
