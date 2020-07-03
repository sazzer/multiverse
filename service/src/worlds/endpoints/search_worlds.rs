use super::model::WorldsResponse;
use crate::{
    http::{pagination::PaginationRequest, sorts::SortFieldsRequest},
    users::endpoints::model::UserLink,
    worlds::{WorldSortField, WorldsFilters, WorldsService},
};
use rocket::{get, State};

/// Handler to search the existing worlds
///
/// # Parameters
/// - `worlds_service` - The worlds service to use
/// - `owner` - The owner to filter against
/// - `url_slug` - The URL Slug to filter against
/// - `sort` - The sort ordering to apply
/// - `pagination` - The pagination controls to apply
///
/// # Returns
/// The details of the worlds that matched the search
#[tracing::instrument(name = "GET /worlds", skip(worlds_service))]
#[get("/worlds?<owner>&<url_slug>&<sort>&<pagination..>")]
pub fn search_worlds(
    worlds_service: State<WorldsService>,
    owner: Option<UserLink>,
    url_slug: Option<String>,
    sort: SortFieldsRequest<WorldSortField>,
    pagination: PaginationRequest,
) -> WorldsResponse {
    tracing::debug!("Searching worlds");

    let worlds = worlds_service.search_worlds(
        &WorldsFilters {
            owner: owner.map(|link| link.into()),
            url_slug,
            ..Default::default()
        },
        &sort,
        &pagination,
    );

    tracing::debug!(worlds = ?worlds, "Found worlds");

    WorldsResponse(worlds)
}
