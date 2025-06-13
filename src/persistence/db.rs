use sqlx::{
    migrate::MigrateDatabase,
    Sqlite,
    SqlitePool,
    Pool
};

use super::DB_URL;
//use super::super::domain::dinosaur::Dinosaur;

async fn open_or_create() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        tracing::info!("Creating database {DB_URL}");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        tracing::info!("Creating database {DB_URL}");
    }
}

pub async fn connect() -> Pool<Sqlite> {
    open_or_create().await;
    SqlitePool::connect(DB_URL).await.expect("DB should already exist")
}

pub async fn insert(dino: &str, pool: &Pool<Sqlite>) {
    let _result = sqlx::query("INSERT INTO dinosaurs (species) VALUES (?)")
        .bind(dino)
        .execute(pool)
        .await
        .expect("The dinosaurs table with species column should exist");
    tracing::info!("Inserted entry into db");
}

//pub async fn query_dinos(pool: &Pool<Sqlite>) -> Vec<Dinosaur> {
    //let dinosaurs = sqlx::query_as::<_, Dinosaur>("SELECT id, species FROM dinosaurs")
        //.fetch_all(pool)
        //.await
        //.unwrap();
    //tracing::info!("Queried table for all dinosaurs");

    //dinosaurs
//}

pub async fn create_table(pool: &Pool<Sqlite>) {
    let query_result = sqlx::query("CREATE TABLE IF NOT EXISTS dinosaurs (id INTEGER PRIMARY KEY NOT NULL, species VARCHAR(250) NOT NULL, taxonomy VARCHAR(250) NOT NULL);")
        .execute(pool).await
        .expect("Database pool should be connected");
    tracing::info!("Created dinosaurs table result {query_result:?}");
}
