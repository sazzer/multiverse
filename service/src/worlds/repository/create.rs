use super::{SaveWorldError, WorldRepository};
use crate::worlds::model::*;
use chrono::Utc;
use uuid::Uuid;

impl WorldRepository {
    /// Create a new world in the database
    ///
    /// # Parameters
    /// - `world` - The world to create
    ///
    /// # Returns
    /// The newly created world
    ///
    /// # Errors
    /// Any errors that occurred creating the new world
    #[tracing::instrument(skip(self))]
    pub fn create(&self, world: WorldData) -> Result<WorldModel, SaveWorldError> {
        let id = WorldID::default();
        let now = Utc::now();
        let version = Uuid::new_v4();

        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");

        let new_world = connection.query_one("INSERT INTO worlds(world_id, version, created, updated, owner_id, name, description, url_slug) VALUES($1, $2, $3, $3, $4, $5, $6, $7) RETURNING *", 
            &[
                &id,
                &version,
                &now,
                &world.owner,
                &world.name,
                &world.description,
                &world.url_slug,
            ])
            .map(|row| self.parse_row(&row))?;

        Ok(new_world)
    }
}
