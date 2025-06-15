use sqlx::{
    migrate::MigrateDatabase,
    Sqlite,
    SqlitePool,
    Pool
};

use super::DB_URL;

async fn open_or_create() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        tracing::info!("Creating database {DB_URL}");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => tracing::info!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        tracing::info!("Creating database {DB_URL}");
    }
}

pub async fn connect() -> Pool<Sqlite> {
    open_or_create().await;
    let pool = SqlitePool::connect(DB_URL).await.expect("DB should already exist");
    tracing::info!("Connected to {}", DB_URL);
    pool
}

#[allow(unused,dead_code)]
pub async fn insert(dino: &str, pool: &Pool<Sqlite>) {
    let _result = sqlx::query("INSERT INTO dinosaurs (species) VALUES (?)")
        .bind(dino)
        .execute(pool)
        .await
        .expect("The dinosaurs table with species column should exist");
    tracing::info!("Inserted entry into db");
}

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
