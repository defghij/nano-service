use axum::{
    extract::{Path, State},
    Json,
    http::status::StatusCode,
    response::IntoResponse
};
use uuid::Uuid;
use utoipa::path as openapi_path;

use crate::{
    domain::dinosaur::Dinosaur,
    dto::dinosaur::*,
    AppState
};


pub mod crud {
    use super::*; 
    #[openapi_path(
        post,
        path = "/dinosaurs",
        request_body = CreateDinosaur,
        responses(
            (status = 201, description = "Dinosaur created", body = Uuid)
        )
    )]
    #[axum_macros::debug_handler]
    pub async fn create(State(pool): State<AppState>, Json(payload): Json<CreateDinosaur>) -> Json<Uuid> {
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
    pub async fn read(Path(id): Path<Uuid>, State(pool): State<AppState>) -> impl IntoResponse {
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
        put,
        path = "/dinosaurs/{id}",
        params(
            ("id" = Uuid, Path, description = "Dinosaur ID")
        ),
        request_body = CreateDinosaur,
        responses(
            (status = 200, description = "Dinosaur updated")
        )
    )]
    #[axum_macros::debug_handler]
    pub async fn update(Path(id): Path<Uuid>, State(pool): State<AppState>, Json(payload): Json<CreateDinosaur>) {
        tracing::debug!("Request: update dinosaur {} to ({:?}, {:?})", id, &payload.species, &payload.taxonomy);
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
    #[axum_macros::debug_handler]
    pub async fn delete(Path(id): Path<Uuid>, State(pool): State<AppState>) {
        tracing::debug!("Request: delete dinosaur {}", id);
        sqlx::query("DELETE FROM dinosaurs WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .unwrap();
    }

    #[openapi_path(
        get,
        path = "/dinosaurs",
        responses(
            (status = 200, description = "Returned all found dinosaurs", body = [Dinosaur]),
        )
    )]
    pub async fn list(State(pool): State<AppState>) -> Json<Vec<Dinosaur>> {
        tracing::debug!("Request: list all dinosaurs");
        let dinos = sqlx::query_as::<_, Dinosaur>("SELECT * FROM dinosaurs")
            .fetch_all(&*pool)
            .await
            .expect("Pool should be available and types are valid");
        Json(dinos)
    }

}

pub mod patch {
    use super::*;

    #[openapi_path(
        patch,
        path = "/dinosaurs/{id}/species",
        params(
            ("id" = Uuid, Path, description = "Dinosaur ID")
        ),
        request_body = UpdateSpecies,
        responses(
            (status = 200, description = "Dinosaur species updated")
        )
    )]
    #[axum_macros::debug_handler]
    pub async fn species(Path(id): Path<Uuid>, State(pool): State<AppState>, Json(payload): Json<UpdateSpecies>) {
        tracing::debug!("Request to update species of {} to {:?}", id, &payload.species);
        sqlx::query("
            UPDATE dinosaurs SET taxonomy = ? WHERE id = ?")
            .bind(&payload.species)
            .bind(id)
            .execute(&*pool)
            .await
            .unwrap();
    }

    #[openapi_path(
        patch,
        path = "/dinosaurs/{id}/taxonomy",
        params(
            ("id" = Uuid, Path, description = "Dinosaur ID")
        ),
        request_body = UpdateTaxonomy,
        responses(
            (status = 200, description = "Dinosaur taxonomy updated")
        )
    )]
    #[axum_macros::debug_handler]
    pub async fn taxonomy(Path(id): Path<Uuid>, State(pool): State<AppState>, Json(payload): Json<UpdateTaxonomy>) {
        tracing::debug!("Request to update taxonomy of {} with {:?}", id, &payload.taxonomy);
        sqlx::query("
            UPDATE dinosaurs SET taxonomy = ? WHERE id = ?")
            .bind(&payload.taxonomy)
            .bind(id)
            .execute(&*pool)
            .await
            .unwrap();
    }
}
