use crate::users::{UserID, UserModel, Username, UsersService};

impl UsersService {
    /// Find a user by it's ID
    ///
    /// # Parameters
    /// - `id` - The User ID to look up
    ///
    /// # Returns
    /// The user if it was found. `None` if not.
    pub fn find_user_by_id(&self, id: &UserID) -> Option<UserModel> {
        self.repository.find_user_by_id(id)
    }

    /// Find a user by it's username
    ///
    /// # Parameters
    /// - `username` - The username to look up
    ///
    /// # Returns
    /// The user if it was found. `None` if not.
    pub fn find_user_by_username(&self, username: &Username) -> Option<UserModel> {
        self.repository.find_user_by_username(username)
    }
}
