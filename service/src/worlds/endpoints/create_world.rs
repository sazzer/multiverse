use crate::{http::problem::Problem, worlds::WorldsService};
use rocket::{post, State};

/// Handler to create a new World
///
/// # Parameters
/// - `worlds_service` - The worlds service to use
///
/// # Returns
/// The newly created world details, or a Problem if the creation failed
#[tracing::instrument(name = "GET /usernames/{id}", skip(worlds_service))]
#[post("/worlds")]
pub fn create_world(worlds_service: State<WorldsService>) -> Result<String, Problem> {
    todo!()
}
