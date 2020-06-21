use crate::{
    http::problem::Problem,
    worlds::{
        endpoints::{errors::WorldProblemType, model::WorldResponse},
        WorldID, WorldsService,
    },
};
use rocket::{get, http::Status, State};

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
    worlds_service
        .find_world_by_id(id)
        .ok_or_else(|| Problem::new(WorldProblemType::UnknownWorldID, Status::NotFound))
        .map(|world| WorldResponse(world))
}
