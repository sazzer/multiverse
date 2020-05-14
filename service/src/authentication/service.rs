use crate::users::UsersService;

/// Service for authenticating a user
#[derive(Clone)]
pub struct AuthenticationService {
    /// The Users Service, to create and retrieve user records
    _users_service: UsersService,
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
        AuthenticationService {
            _users_service: users_service,
        }
    }
}
