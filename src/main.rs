use tracing::Level;
use sqlx::SqlitePool;

use std::sync::Arc;

mod domain;
mod persistence;
mod interface;
mod observability;
mod dto;

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
    

    let app = interface::routes(Arc::new(pool));

    let listener = tokio::net::TcpListener::bind(interface::URL_BASE).await.unwrap();
    tracing::info!("Bound lister to {}", interface::URL_BASE);
    tracing::info!("View API at {}/{}",interface::URL_BASE, interface::OPENAPI_PATH);

    axum::serve(listener, app).await.unwrap();
}
