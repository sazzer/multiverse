use crate::infrastructure::database::Database;
use std::sync::Arc;

/// Application Configuration for the Worlds module
pub struct WorldsConfig {}

impl WorldsConfig {
    /// Create the Configuration for the Worlds Module
    ///
    /// # Returns
    /// The Worlds Config object
    pub fn new(_database: Database) -> Self {
        Self {}
    }

    /// Generate the configuration callback needed for the HTTP Server to actually add the Worlds endpoints
    /// to the server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the Worlds endpoints
    pub fn configure(&self) -> Arc<dyn Fn(rocket::Rocket) -> rocket::Rocket + Send + Sync> {
        Arc::new(move |config| config)
    }
}
