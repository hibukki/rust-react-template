use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

pub type DbPool = SqlitePool;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    // Ensure the database file exists for SQLite
    if database_url.starts_with("sqlite:") {
        let path = database_url.trim_start_matches("sqlite:");
        if path != ":memory:" && !Path::new(path).exists() {
            // Create parent directories if needed
            if let Some(parent) = Path::new(path).parent() {
                std::fs::create_dir_all(parent).ok();
            }
            // Create empty file
            std::fs::File::create(path).ok();
        }
    }

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("../../migrations").run(pool).await
}
