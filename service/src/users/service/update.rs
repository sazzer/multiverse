use crate::users::{UserData, UserID, UserModel, UsersService};

impl UsersService {
    /// Find a user by it's ID and update the data associated with it
    ///
    /// # Parameters
    /// - `id` - The User ID to look up
    ///
    /// # Returns
    /// The user if it was found. `None` if not.
    pub fn update_user<E>(
        &self,
        id: &UserID,
        updater: &dyn Fn(UserData) -> Result<UserData, E>,
    ) -> Result<UserModel, UpdateError<E>> {
        let user = self.repository.find_user_by_id(id).unwrap();

        let updated_user = updater(user.data).map_err(|e| UpdateError::ClientError(e))?;

        self.repository
            .update(id, updated_user)
            .map_err(|_| UpdateError::RepositoryError)
    }
}

/// An error occurred updating the user
pub enum UpdateError<E> {
    /// The error occurred within the repository
    RepositoryError,

    /// The error occurred within the client callback
    ClientError(E),
}
