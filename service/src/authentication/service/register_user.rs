use crate::authentication::{AuthenticatedUser, AuthenticationService};
use crate::users::{CreateUserError, UserData};

/// Errors that can occur when registering a new user
#[derive(Debug, thiserror::Error)]
pub enum RegisterError {
    /// An unknown error occurred
    #[error("An unknown error occurred")]
    UnknownError,

    #[error("The username is already registered")]
    DuplicateUsername,
}

impl AuthenticationService {
    /// Register a new user with the system
    ///
    /// # Parameters
    /// - `user` - The details of the user to register
    ///
    /// # Returns
    /// The authenticated user, with both the newly created user details and the authentication token
    ///
    /// # Errors
    /// Any errors that happen from registering a user
    pub fn register_user(&self, user: UserData) -> Result<AuthenticatedUser, RegisterError> {
        // Call the User Service to create a new User record with the given data
        let user = self.users_service.create_user(user).map_err(|e| {
            tracing::warn!("Failed to create user: {:?}", e);
            e
        })?;

        // Call the Authorization Service to create a new Token for the User
        let authorization = self.authorization_service.generate_authorization(&user);

        // Return the Authenticated User
        // todo!()
        Ok(AuthenticatedUser {
            user,
            authorization,
        })
    }
}

impl From<CreateUserError> for RegisterError {
    fn from(e: CreateUserError) -> Self {
        match e {
            CreateUserError::DuplicateUsername => RegisterError::DuplicateUsername,
            _ => RegisterError::UnknownError,
        }
    }
}
