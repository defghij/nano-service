use tracing::Level;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

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

    //let pool = persistence::db::connect().await;
    //persistence::db::create_table(&pool).await;


    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://sqlite.db")
        .await
        .expect("Failed to connect to DB");

    let app = interfaces::api::app(Arc::new(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
