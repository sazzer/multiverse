use super::WorldsService;
use crate::{
    model::{Page, Pagination, SortFields},
    users::UserID,
    worlds::{WorldID, WorldModel, WorldSortField},
};

/// Filters to apply when searching for worlds
#[derive(Debug, Default)]
pub struct WorldsFilters {
    /// Only include worlds owned by this user
    pub owner: Option<UserID>,
}

impl WorldsService {
    /// Look up a world by it's unique ID
    ///
    /// # Parameters
    /// - `id` - The ID of the World to find
    ///
    /// # Returns
    /// The World, or `None` if it wasn't found
    pub fn find_world_by_id(&self, id: WorldID) -> Option<WorldModel> {
        self.repository.find_world_by_id(id)
    }

    /// Sarch for worlds that match the given criteria
    ///
    /// # Parameters
    /// - `filters` - The filters to apply to the results
    /// - `sorts` - The order in which to sort the results
    /// - `pagination` - The pagination details for which page of results are wanted
    ///
    /// # Returns
    /// The requested page of results
    pub fn search_worlds(
        &self,
        filters: &WorldsFilters,
        sorts: &SortFields<WorldSortField>,
        pagination: &Pagination,
    ) -> Page<WorldModel> {
        tracing::debug!(filters = ?filters, sorts = ?sorts, pagination = ?pagination, "Searching worlds");
        todo!()
    }
}
