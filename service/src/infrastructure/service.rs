use super::server::Server;
use crate::{
    authentication::configure::AuthenticationConfig,
    infrastructure::{database, healthchecker::configure::HealthcheckConfig},
    users::configure::UsersConfig,
};
use std::sync::Arc;

/// The actual service that represents the entire system.
pub struct Service {
    server: Server,
}

impl Service {
    /// Construct a new service, configured with the provided settings and ready to run
    ///
    /// # Parameters
    /// - `settings` - The settings needed to build the application
    ///
    /// # Returns
    /// The built service, ready to work with
    pub fn new(settings: crate::Settings) -> Self {
        tracing::info!(settings = ?settings, "Building service");

        let database = database::Database::new(settings.database_url);
        database::migrate::migrate_database(&database).expect("Failed to migrate database");

        let users = UsersConfig::new(database.clone());
        let authorization = crate::authorization::configure::AuthorizationConfig::new();
        let authentication = AuthenticationConfig::new(users.users_service.clone());

        let healthchecks = HealthcheckConfig::default().with_component("db", Arc::new(database));

        Service {
            server: Server::new(vec![
                healthchecks.configure(),
                users.configure(),
                authorization.configure(),
                authentication.configure(),
            ]),
        }
    }

    /// Start the service listening on the given HTTP port and accepting incoming connections
    ///
    /// # Parameters
    /// - `port` - The port to listen on
    pub fn start(&self, port: u16) {
        tracing::info!(port = port, "Starting service");
        self.server.start(port);
    }

    /// Get a test client used to test the server
    ///
    /// # Returns
    /// The test client
    pub fn test_client(&self) -> rocket::local::Client {
        self.server.test_client()
    }
}
