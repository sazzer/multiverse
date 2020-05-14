use actix_web::web;
use std::sync::Arc;

/// Application Configuration for the Authentication module
pub struct AuthenticationConfig {}

impl AuthenticationConfig {
    /// Create the Configuration for the Authentication Module
    ///
    /// # Returns
    /// The Authentication Config object
    pub fn new() -> Self {
        Self {}
    }
    /// Generate the configuration callback needed for the HTTP Server to actually add the Authentication endpoints
    /// to the server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the Authentication endpoints
    pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
        Arc::new(move |config| {
            config.service(super::endpoints::register_user);
        })
    }
}
