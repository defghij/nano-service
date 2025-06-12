use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct Dinosaur {
    pub id: u32,
    pub species: String,
}
