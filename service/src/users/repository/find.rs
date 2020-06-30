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

    /// Find all of the users that have one of the given collection of IDs
    ///
    /// # Parameters
    /// - `ids` - The IDs to look up
    ///
    /// # Returns
    /// The users that were found. May be empty if no users with the given IDs were found.
    pub fn find_users_by_id(&self, ids: &[&UserID]) -> Vec<UserModel> {
        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");

        let user_id_binds = ids
            .iter()
            .enumerate()
            .map(|(index, _)| index)
            .map(|index| format!("${}", index + 1))
            .collect::<Vec<String>>()
            .join(",");
        let query = format!("SELECT * FROM users WHERE user_id IN ({})", user_id_binds);

        let binds: Vec<&(dyn postgres::types::ToSql + Sync)> = ids
            .into_iter()
            .map(|&id| id as &(dyn postgres::types::ToSql + Sync))
            .collect();

        connection
            .query(query.as_str(), &binds[..])
            .map(|rows| rows.iter().map(|row| self.parse_row(row)).collect())
            .expect("Failed to select worlds matching query")
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
