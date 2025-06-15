use axum::{
    extract::{Path, State},
    Json, Router,
    routing::get,
    http::status::StatusCode,
    response::IntoResponse
};
use uuid::Uuid;
use utoipa::{
    OpenApi, 
    path as openapi_path
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{interfaces::ApiDoc, AppState};

use crate::domain::dinosaur::{CreateDinosaur, Dinosaur};


pub fn app(state: AppState) -> Router {
    tracing::info!("Creating routes with webserver");
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/dinosaurs", get(list_dinosaurs).post(create_dinosaur))
        .route("/dinosaurs/:id", get(get_dinosaur).put(update_dinosaur).delete(delete_dinosaur))
        .with_state(state)
}


#[openapi_path(
    get,
    path = "/dinosaurs",
    responses(
        (status = 200, description = "List dinosaurs", body = [Dinosaur])
    )
)]
async fn list_dinosaurs(State(pool): State<AppState>) -> Json<Vec<Dinosaur>> {
    tracing::debug!("Request to list all dinosaurs");
    let dinos = sqlx::query_as::<_, Dinosaur>("SELECT * FROM dinosaurs")
        .fetch_all(&*pool)
        .await
        .expect("Pool should be available and types are valid");
    Json(dinos)
}

#[openapi_path(
    get,
    path = "/dinosaurs/{id}",
    params(
        ("id" = Uuid, Path, description = "Dinosaur UUID")
    ),
    responses(
        (status = 200, body = Dinosaur),
        (status = 404, description = "Dinosaur not found")
    )
)]
#[axum_macros::debug_handler]
async fn get_dinosaur(Path(id): Path<Uuid>, State(pool): State<AppState>) -> impl IntoResponse {
    tracing::debug!("Request for dinosaur with id {}", id);
    sqlx::query_as::<_, Dinosaur>("SELECT * FROM dinosaurs WHERE id = ?")
        .bind(id)
        .fetch_optional(&*pool)
        .await
        .ok()
        .flatten()
        .map(Json)
        .map_or_else(|| StatusCode::NOT_FOUND.into_response(),
                     |query_result| query_result.into_response())
}

#[openapi_path(
    post,
    path = "/dinosaurs",
    request_body = CreateDinosaur,
    responses(
        (status = 201, description = "Dinosaur created", body = Uuid)
    )
)]
async fn create_dinosaur(State(pool): State<AppState>, Json(payload): Json<CreateDinosaur>) -> Json<Uuid> {
    tracing::debug!("Request to create dinosaur ({:?}, {:?})", &payload.species, &payload.taxonomy);
    let id = Uuid::new_v4();
    let dino = Dinosaur { id, species: payload.species, taxonomy: payload.taxonomy };
    sqlx::query("INSERT INTO dinosaurs (id, species, taxonomy) VALUES (?, ?, ?)")
        .bind(dino.id)
        .bind(&dino.species)
        .bind(&dino.taxonomy)
        .execute(&*pool)
        .await
        .expect("Data structure should be well formed");

    let dinos = sqlx::query_as::<_, Dinosaur>("SELECT * FROM dinosaurs")
        .fetch_all(&*pool)
        .await
        .expect("Database pool should exist for querying");
    tracing::debug!("list of dinosaurs after insert: {:?}", dinos);

    Json(dino.id)
}

#[openapi_path(
    put,
    path = "/dinosaurs/{id}",
    params(
        ("id" = Uuid, Path, description = "Dinosaur ID")
    ),
    request_body = Dinosaur,
    responses(
        (status = 200, description = "Dinosaur updated")
    )
)]
async fn update_dinosaur(Path(id): Path<Uuid>, State(pool): State<AppState>, Json(payload): Json<Dinosaur>) {
    tracing::debug!("Request to update dinosaur {} with ({:?}, {:?})", id, &payload.species, &payload.taxonomy);
    sqlx::query("UPDATE dinosaurs SET species = ?, taxonomy = ? WHERE id = ?")
        .bind(id)
        .bind(&payload.species)
        .bind(&payload.taxonomy)
        .execute(&*pool)
        .await
        .unwrap();
}

#[openapi_path(
    delete,
    path = "/dinosaurs/{id}",
    params(
        ("id" = Uuid, Path, description = "Dinosaur ID")
    ),
    responses(
        (status = 200, description = "Dinosaur deleted"),
        (status = 404, description = "Dinosaur not found")
    )
)]
async fn delete_dinosaur(Path(id): Path<Uuid>, State(pool): State<AppState>) {
    tracing::debug!("Request to delete dinosaur {}", id);
    sqlx::query("DELETE FROM dinosaurs WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .unwrap();
}
