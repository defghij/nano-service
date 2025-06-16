pub mod dinosaur;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, patch};
use axum::{Json, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

use crate::{
    domain::dinosaur::{Dinosaur, Taxonomy},
    dto::dinosaur::*,
    AppState
};

pub const URL_BASE: &str = "0.0.0.0:3000";
pub const OPENAPI_PATH: &str = "docs";

pub fn routes(state: AppState) -> Router {
    tracing::info!("Creating routes with webserver");
    Router::new()
        .merge(SwaggerUi::new(format!("/{OPENAPI_PATH}"))
            .url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/dinosaurs", 
            get(dinosaur::crud::list)
            .post(dinosaur::crud::create))
        .route("/dinosaurs/:id", 
            get(dinosaur::crud::read)
            .delete(dinosaur::crud::delete)
            .put(dinosaur::crud::update)
        )
        .route("/dinosaurs/:id/taxonomy", 
            patch(dinosaur::patch::taxonomy)
        )
        .route("/dinosaurs/:id/species", 
            patch(dinosaur::patch::species)
        )
        .with_state(state)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        dinosaur::crud::create,
        dinosaur::crud::read,
        dinosaur::crud::update,
        dinosaur::crud::delete,
        dinosaur::crud::list,
        dinosaur::patch::taxonomy,
        dinosaur::patch::species,
    ),
    components(schemas(Dinosaur, CreateDinosaur,
                       UpdateSpecies, Taxonomy, UpdateTaxonomy)),
    tags(
        (name = "Dinosaurs", description = "Dinosaur management API")
    )
)]
pub struct ApiDoc;


pub trait _CrudAPI {
    type Create;
    type Row;
    type Id;

    async fn create(state: State<AppState>, payload: Json<Self::Create>) -> Json<Uuid>;
    async fn read(id: Path<Self::Id>, state: State<AppState>) -> impl IntoResponse;
    async fn update(id: Path<Self::Id>, state: State<AppState>, payload: Json<Self::Row>);
    async fn delete(id: Path<Self::Id>, state: State<AppState>);
    async fn list(state: State<AppState>) -> Json<Vec<Self::Row>>;
}
