mod authenticate_user;
mod register_user;

use crate::{authorization::AuthorizationService, users::UsersService};
pub use register_user::RegisterError;

/// Service for authenticating a user
#[derive(Clone)]
pub struct AuthenticationService {
    /// The Users Service, to create and retrieve user records
    users_service: UsersService,
    /// The Authorization Service, to generate authorization records for users
    authorization_service: AuthorizationService,
}

impl AuthenticationService {
    /// Create a new instance of the Authentication Service
    ///
    /// # Parameters
    /// - `users_service` - The users service
    /// - `authorization_service` - The authorization service
    ///
    /// # Returns
    /// The Authentication Service ready to use
    pub(super) fn new(
        users_service: UsersService,
        authorization_service: AuthorizationService,
    ) -> Self {
        AuthenticationService {
            users_service,
            authorization_service,
        }
    }
}
