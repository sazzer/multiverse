use super::AuthorizationService;
use std::sync::Arc;

/// Application Configuration for the Authorization module
pub struct AuthorizationConfig {
    authorization_service: AuthorizationService,
}

impl AuthorizationConfig {
    /// Create the Configuration for the Authorization Module
    ///
    /// # Returns
    /// The Authorization Config object
    pub fn new() -> Self {
        Self {
            authorization_service: AuthorizationService::new(),
        }
    }
    /// Generate the configuration callback needed for the HTTP Server to actually add the Authorization endpoints
    /// to the server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the Authorization endpoints
    pub fn configure(&self) -> Arc<dyn Fn(rocket::Rocket) -> rocket::Rocket + Send + Sync> {
        let authorization_service = self.authorization_service.clone();
        Arc::new(move |config| config.manage(authorization_service.clone()))
    }
}
