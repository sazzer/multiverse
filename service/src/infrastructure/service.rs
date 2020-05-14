use super::server::{testing::TestResponse, Server};
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
    pub async fn new(settings: crate::Settings) -> Self {
        tracing::info!(settings = ?settings, "Building service");

        let database = database::Database::new(settings.database_url).await;
        database::migrate::migrate_database(&database)
            .await
            .expect("Failed to migrate database");

        let users = UsersConfig::new(database.clone());
        let authentication = AuthenticationConfig::new();

        let healthchecks = HealthcheckConfig::default().with_component("db", Arc::new(database));

        Service {
            server: Server::new(vec![
                healthchecks.configure(),
                users.configure(),
                authentication.configure(),
            ]),
        }
    }

    /// Start the service listening on the given HTTP port and accepting incoming connections
    ///
    /// # Parameters
    /// - `port` - The port to listen on
    pub async fn start(&self, port: u16) {
        tracing::info!(port = port, "Starting service");
        self.server.start(port).await;
    }

    /// Send an HTTP Request in to the service and return the response
    ///
    /// # Parameters
    /// - `request` - The request to send to the service
    ///
    /// # Returns
    /// The HTTP Response
    pub async fn test_request(&self, request: actix_http::Request) -> TestResponse {
        self.server.test_request(request).await
    }
}
