mod register_user;

use crate::users::UsersService;
pub use register_user::RegisterError;

/// Service for authenticating a user
#[derive(Clone)]
pub struct AuthenticationService {
    /// The Users Service, to create and retrieve user records
    users_service: UsersService,
}

impl AuthenticationService {
    /// Create a new instance of the Authentication Service
    ///
    /// # Parameters
    /// - `users_service` - The users service
    ///
    /// # Returns
    /// The Authentication Service ready to use
    pub fn new(users_service: UsersService) -> Self {
        AuthenticationService { users_service }
    }
}
