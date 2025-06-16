use std::sync::Arc;

use axum::{body::Body, http::{Request, StatusCode}, Router};
//use tower::ServiceExt; // for `app.oneshot()`
//use uuid::Uuid;
//use serde_json::json;
use sqlx::SqlitePool;
use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

use crate::{
    dto::dinosaur::CreateDinosaur,
    persistence::{
        db::create_table,
        DB_URL
    },
    interface,
    domain::dinosaur::Taxonomy
};

#[tokio::test]
async fn test_create_and_list_dinosaur() {
    // Set up in-memory database for testing.
    let pool = SqlitePool::connect(DB_URL).await.expect("DB should already exist or be created");
    create_table(&pool).await;

    let app = interface::routes(Arc::new(pool));

    let payload = serde_json::json!(
        CreateDinosaur {
            species: String::from("T-rex"),
            taxonomy: Taxonomy::Genus
        }
    );

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/dinosaurs")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/dinosaurs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

