use sqlx::{
    migrate::MigrateDatabase,
    Sqlite,
    SqlitePool,
    Pool
};

use super::DB_URL;

/// Checks if a database at the specified url already exists and 
/// if it does not then it creates a new one.
async fn create_database() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        tracing::debug!("Creating database: {DB_URL}");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => tracing::debug!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        tracing::debug!("Using existing database: {DB_URL}");
    }
}

/// Connects to a local database. It will create the database 
/// if one does not already exist.
pub async fn connect() -> Pool<Sqlite> {
    create_database().await;
    let pool = SqlitePool::connect(DB_URL).await.expect("DB should already exist");
    tracing::info!("Connected to {}", DB_URL);
    pool
}

/// Takes the provided pool drops the existing table, if any
/// exists, and recreates it.
pub async fn create_table(pool: &Pool<Sqlite>) {
    let query_result = sqlx::query("
        DROP TABLE dinosaurs;      -- Remove old one first
        CREATE TABLE IF NOT EXISTS dinosaurs (
            id TEXT PRIMARY KEY NOT NULL, 
            species VARCHAR(250) NOT NULL, 
            taxonomy VARCHAR(250) NOT NULL
            );
        ")
        .execute(pool).await
        .expect("Database pool should be connected");
    tracing::info!("Created dinosaurs table result {query_result:?}");
}
