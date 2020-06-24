use crate::{
    http::{pagination::PaginationRequest, problem::Problem, sorts::SortFieldsRequest},
    model::ParseSortFieldError,
    worlds::{endpoints::model::WorldResponse, WorldSortField, WorldsService},
};
use rocket::{get, State};

/// Handler to search the existing worlds
///
/// # Parameters
/// - `worlds_service` - The worlds service to use
///
/// # Returns
/// The details of the worlds that matched the search
#[tracing::instrument(name = "GET /worlds", skip(worlds_service))]
#[get("/worlds?<sort>&<pagination..>")]
pub fn search_worlds(
    worlds_service: State<WorldsService>,
    sort: Option<Result<SortFieldsRequest<WorldSortField>, ParseSortFieldError>>,
    pagination: PaginationRequest,
) -> Result<WorldResponse, Problem> {
    tracing::debug!("Searching worlds");
    todo!()
}
