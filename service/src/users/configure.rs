use super::UsersService;
use actix_web::web;
use std::sync::Arc;

/// Application Configuration for the Users module
pub struct UsersConfig {
    users_service: UsersService,
}

impl UsersConfig {
    /// Create the Configuration for the Users Module
    ///
    /// # Returns
    /// The Users Config object
    pub fn new() -> Self {
        Self {
            users_service: UsersService::new(),
        }
    }
    /// Generate the configuration callback needed for the HTTP Server to actually add the Users endpoints
    /// to the server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the Users endpoints
    pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
        let users_service = self.users_service.clone();
        Arc::new(move |config| {
            config.data(users_service.clone());
            config.service(super::endpoints::lookup_username);
        })
    }
}
