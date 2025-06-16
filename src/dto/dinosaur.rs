use sqlx::{Encode, Decode, Type, FromRow};
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};
//use uuid::Uuid;

use crate::domain::dinosaur::Taxonomy;

#[derive(Clone, Debug)]
#[derive(ToSchema, Serialize, Deserialize, Type, FromRow, Encode, Decode)]
pub struct CreateDinosaur {
    pub species: String,
    pub taxonomy: Taxonomy,
}

#[derive(Clone, Debug)]
#[derive(ToSchema, Serialize, Deserialize, Type, FromRow, Encode, Decode)]
pub struct UpdateSpecies {
    pub species: String,
}

#[derive(Clone, Debug)]
#[derive(ToSchema, Serialize, Deserialize, Type, Encode, Decode)]
pub struct UpdateTaxonomy {
    pub taxonomy: Taxonomy,
}
