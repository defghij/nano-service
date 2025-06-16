use axum::{Router, body::Body};
use http::{Request, StatusCode};
use tower::ServiceExt; // for `app.oneshot()`
use uuid::Uuid;
use serde_json::json;

use your_project::{api::routes, db::connect}; // adjust to match your module

#[tokio::test]
async fn test_create_and_list_dinosaur() {
    // Setup shared DB connection (prefer test DB or in-memory sqlite for isolation)
    let state = connect().await;
    let app = routes(state.clone());

    // -- Test creation --
    let payload = json!({
        "species": "Pachycephalosaurus",
        "taxonomy": "genus"
    });

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

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json.is_array());
    assert!(json.as_array().unwrap().iter().any(|d| d["species"] == "Pachycephalosaurus"));
}

