pub mod api;

use utoipa::OpenApi;
//use utoipa_swagger_ui::SwaggerUi;
use crate::domain::dinosaur::{CreateDinosaur, Dinosaur, Taxonomy};

pub const URL: &str = "0.0.0.0:3000";

#[derive(OpenApi)]
#[openapi(
    paths(
        api::list_dinosaurs,
        api::get_dinosaur,
        api::create_dinosaur,
        api::update_dinosaur,
        api::delete_dinosaur,
    ),
    components(schemas(CreateDinosaur, Dinosaur, Taxonomy)),
    tags(
        (name = "Dinosaurs", description = "Dinosaur management API")
    )
)]
pub struct ApiDoc;
