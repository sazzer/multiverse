use super::UserRepository;
use crate::users::{UserID, UserModel, Username};

impl UserRepository {
    /// Look up the user record that has the provided ID
    ///
    /// # Parameters
    /// - `id` - The ID to look up
    ///
    /// # Returns
    /// The user that was found, if there was one. `None` if one wasn't found
    #[tracing::instrument(skip(self))]
    pub fn find_user_by_id(&self, id: &UserID) -> Option<UserModel> {
        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");
        connection
            .query_opt("SELECT * FROM users WHERE user_id = $1", &[id])
            .expect("Failed to query for user by ID")
            .map(|row| self.parse_row(&row))
    }

    /// Look up the user record that has the provided Username
    ///
    /// # Parameters
    /// - `username` - The username to look up
    ///
    /// # Returns
    /// The user that was found, if there was one. `None` if one wasn't found
    #[tracing::instrument(skip(self))]
    pub fn find_user_by_username(&self, username: &Username) -> Option<UserModel> {
        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");
        connection
            .query_opt("SELECT * FROM users WHERE username = $1", &[username])
            .expect("Failed to query for user by username")
            .map(|row| self.parse_row(&row))
    }
}
