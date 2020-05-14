use super::AuthenticationService;
use crate::users::UsersService;
use actix_web::web;
use std::sync::Arc;

/// Application Configuration for the Authentication module
pub struct AuthenticationConfig {
    authentication_service: AuthenticationService,
}

impl AuthenticationConfig {
    /// Create the Configuration for the Authentication Module
    ///
    /// # Returns
    /// The Authentication Config object
    pub fn new(users_service: UsersService) -> Self {
        let authentication_service = AuthenticationService::new(users_service);
        Self {
            authentication_service,
        }
    }
    /// Generate the configuration callback needed for the HTTP Server to actually add the Authentication endpoints
    /// to the server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the Authentication endpoints
    pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
        let authentication_service = self.authentication_service.clone();
        Arc::new(move |config| {
            config.data(authentication_service.clone());
            config.service(super::endpoints::register_user);
        })
    }
}
