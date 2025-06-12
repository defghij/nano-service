use tracing::Level;

mod domain;
mod persistence;
mod observability;

use domain::dinosaur::Dinosaur;
use persistence::db;
use observability::setup_tracing;


#[tokio::main]
async fn main() {
    setup_tracing(Level::INFO);

    // Set up database connection
    let pool = db::connect().await;
    db::create_table(&pool).await;

    // Add elements to the database
    db::insert("Stegasaurus", &pool).await;
    db::insert("Trex", &pool).await;
    db::insert("Raptor", &pool).await;
    db::insert("Pidgeon", &pool).await;

    // Query database
    db::query_dinos(&pool).await
        .iter()
        .for_each(|dino: &Dinosaur| {
            println!("[{}] species: {}", dino.id, dino.species);
        });
}
