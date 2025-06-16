use sqlx::{Encode, Decode, Type, FromRow};
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;



#[derive(Clone, Debug)]
#[derive(ToSchema, Serialize, Deserialize, Type, FromRow, Encode, Decode)]
pub struct Dinosaur {
    pub id: Uuid,
    pub species: String,
    pub taxonomy: Taxonomy,
}

#[derive(Clone, Debug)]
#[derive(ToSchema, Serialize, Deserialize,Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all="lowercase")]
#[serde(rename_all="lowercase")]
pub enum Taxonomy {
    Genus, 
    Species
}
