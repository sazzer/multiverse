use super::WorldsService;
use crate::worlds::{WorldID, WorldModel};

impl WorldsService {
    /// Look up a world by it's unique ID
    ///
    /// # Parameters
    /// - `id` - The ID of the World to find
    ///
    /// # Returns
    /// The World, or `None` if it wasn't found
    pub fn find_world_by_id(&self, id: WorldID) -> Option<WorldModel> {
        None
    }
}
