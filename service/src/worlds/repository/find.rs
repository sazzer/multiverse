use super::WorldRepository;
use crate::worlds::model::*;

impl WorldRepository {
    /// Find a world in the database
    ///
    /// # Parameters
    /// - `id` - The ID of the world to find
    ///
    /// # Returns
    /// The world world, if it exists
    #[tracing::instrument(skip(self))]
    pub fn find_world_by_id(&self, id: WorldID) -> Option<WorldModel> {
        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");
        connection
            .query_opt("SELECT * FROM worlds WHERE world_id = $1", &[&id])
            .expect("Failed to query for world by ID")
            .map(|row| self.parse_row(&row))
    }
}
