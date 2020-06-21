use crate::{
    http::problem::Problem,
    worlds::{endpoints::model::WorldResponse, WorldID, WorldsService},
};
use rocket::{get, State};

/// Handler to get an existing World by ID
///
/// # Parameters
/// - `worlds_service` - The worlds service to use
///
/// # Returns
/// The details of the world
#[tracing::instrument(name = "GET /worlds/{id}", skip(worlds_service))]
#[get("/worlds/<id>")]
pub fn get_world(
    worlds_service: State<WorldsService>,
    id: WorldID,
) -> Result<WorldResponse, Problem> {
    tracing::debug!("Looking up world");
    todo!()
}
