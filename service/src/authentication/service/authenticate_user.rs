use crate::authentication::{AuthenticatedUserModel, AuthenticationService};
use crate::users::{Plaintext, Username};

/// Errors that can occur when registering a new user
#[derive(Debug, thiserror::Error)]
pub enum AuthenticateError {
    /// The user was unknown
    #[error("The user was unknown")]
    UnknownUser,

    /// The password was incorrect
    #[error("The password was incorrect")]
    InvalidPassword,
}

impl AuthenticationService {
    /// Authenticate a user
    ///
    /// # Parameters
    /// - `username` - The username to authenticate
    /// - `password` - The password to authenticate
    ///
    /// # Returns
    /// The authenticated user, with both the newly authenticated user details and the authentication token
    ///
    /// # Errors
    /// Any errors that happen from authenticating a user
    pub fn authenticate_user(
        &self,
        username: Username,
        password: Plaintext,
    ) -> Result<AuthenticatedUserModel, AuthenticateError> {
        // Call the User Service to load a new User record with the given data
        let user = self
            .users_service
            .find_user_by_username(&username)
            .ok_or(AuthenticateError::UnknownUser)?;

        // Check that the password matches
        if user.data.password != password {
            Err(AuthenticateError::InvalidPassword)?
        }

        // Call the Authorization Service to create a new Token for the User
        let authorization = self.authorization_service.generate_authorization(&user);

        // Return the Authenticated User
        Ok(AuthenticatedUserModel {
            user,
            authorization,
        })
    }
}
