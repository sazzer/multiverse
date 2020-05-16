use crate::users::repository::SaveUserError;
use crate::users::*;

/// Errors that can occur when creating a new user record
#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    /// An unknown error occurred
    #[error("An unknown error occurred")]
    UnknownError,

    #[error("The username is already registered")]
    DuplicateUsername,
}

impl UsersService {
    /// Create a new user record in the user repository
    ///
    /// # Parameters
    /// - `data` - The data to use for the new user
    ///
    /// # Returns
    /// The newly created user
    ///
    /// # Errors
    /// Any errors that occurred creating the new user
    pub async fn create_user(&self, data: UserData) -> Result<UserModel, CreateUserError> {
        let user = UserModel {
            identity: Default::default(),
            data,
        };
        let user = self.repository.create(user).await?;

        Ok(user)
    }
}

impl From<SaveUserError> for CreateUserError {
    fn from(e: SaveUserError) -> Self {
        match e {
            SaveUserError::DuplicateUsername => CreateUserError::DuplicateUsername,
            _ => CreateUserError::UnknownError,
        }
    }
}