use super::{repository::WorldRepository, WorldsService};
use crate::infrastructure::database::Database;
use std::sync::Arc;

/// Application Configuration for the Worlds module
pub struct WorldsConfig {
    pub worlds_service: WorldsService,
}

impl WorldsConfig {
    /// Create the Configuration for the Worlds Module
    ///
    /// # Returns
    /// The Worlds Config object
    pub fn new(database: Database) -> Self {
        let repository = WorldRepository::new(database);
        Self {
            worlds_service: WorldsService::new(repository),
        }
    }

    /// Generate the configuration callback needed for the HTTP Server to actually add the Worlds endpoints
    /// to the server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the Worlds endpoints
    pub fn configure(&self) -> Arc<dyn Fn(rocket::Rocket) -> rocket::Rocket + Send + Sync> {
        let worlds_service = self.worlds_service.clone();
        Arc::new(move |config| {
            config.manage(worlds_service.clone()).mount(
                "/",
                rocket::routes![super::endpoints::create_world, super::endpoints::get_world],
            )
        })
    }
}
