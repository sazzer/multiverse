use super::AuthenticationService;
use crate::authorization::AuthorizationService;
use crate::users::UsersService;
use rocket::Rocket;
use std::sync::Arc;

/// Application Configuration for the Authentication module
pub struct AuthenticationConfig {
    authentication_service: AuthenticationService,
}

impl AuthenticationConfig {
    /// Create the Configuration for the Authentication Module
    ///
    /// # Parameters
    /// - `users_service` - The users service
    /// - `authorization_service` - The authorization service
    ///
    /// # Returns
    /// The Authentication Config object
    pub fn new(users_service: UsersService, authorization_service: AuthorizationService) -> Self {
        let authentication_service =
            AuthenticationService::new(users_service, authorization_service);
        Self {
            authentication_service,
        }
    }
    /// Generate the configuration callback needed for the HTTP Server to actually add the Authentication endpoints
    /// to the server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the Authentication endpoints
    pub fn configure(&self) -> Arc<dyn Fn(Rocket) -> Rocket + Send + Sync> {
        let authentication_service = self.authentication_service.clone();
        Arc::new(move |config| {
            config.manage(authentication_service.clone()).mount(
                "/",
                rocket::routes![
                    super::endpoints::register_user,
                    super::endpoints::login_user
                ],
            )
        })
    }
}
