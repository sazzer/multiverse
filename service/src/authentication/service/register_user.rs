use crate::authentication::{AuthenticatedUser, AuthenticationService};
use crate::users::UserData;

/// Errors that can occur when registering a new user
#[derive(Debug, thiserror::Error)]
pub enum RegisterError {
    /// An unknown error occurred
    #[error("An unknown error occurred")]
    UnknownError,
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
    pub async fn register_user(&self, user: UserData) -> Result<AuthenticatedUser, RegisterError> {
        // Call the User Service to create a new User record with the given data
        let _user_model = self.users_service.create_user(user).await;

        // Call the Authorization Service to create a new Token for the User

        // Return the Authenticated User
        todo!()
    }
}
