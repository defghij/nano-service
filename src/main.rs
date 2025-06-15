use persistence::DB_URL;
use tracing::Level;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};

use std::sync::Arc;

mod domain;
mod persistence;
mod observability;
mod interfaces;

//use domain::dinosaur::Dinosaur;
use observability::setup_tracing;

type AppState = Arc<SqlitePool>;

#[tokio::main]
async fn main() {
    setup_tracing(Level::DEBUG);

    let pool = persistence::db::connect().await;
    persistence::db::create_table(&pool).await;

    //let db = SqlitePoolOptions::new()
        //.max_connections(5)
        //.connect(persistence::DB_URL)
        //.await
        //.expect("Failed to connect to DB");
    //tracing::info!("Connected to {}", persistence::DB_URL);
    

    let app = interfaces::api::app(Arc::new(pool));

    let listener = tokio::net::TcpListener::bind(interfaces::URL).await.unwrap();
    tracing::info!("Bound lister to {}", interfaces::URL);
    tracing::info!("View API at {}/api", interfaces::URL);

    axum::serve(listener, app).await.unwrap();
}
