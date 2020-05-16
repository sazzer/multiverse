use super::{SaveUserError, UserRepository};
use crate::users::UserModel;

impl UserRepository {
    /// Insert the given user model into the database as a new record
    ///
    /// # Parameters
    /// - `user` - The details to insert into the database
    ///
    /// # Returns
    /// The newly created user
    ///
    /// # Errors
    /// Any errors that occurred creating the new user
    pub fn create(&self, user: UserModel) -> Result<UserModel, SaveUserError> {
        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");
        let new_user = connection.query_one("INSERT INTO users(user_id, version, created, updated, username, display_name, email_address, avatar_url, password) VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *", 
            &[
                &user.identity.id,
                &user.identity.version,
                &user.identity.created,
                &user.identity.updated,
                &user.data.username,
                &user.data.display_name,
                &user.data.email_address,
                &user.data.avatar_url,
                &user.data.password,    
            ])
            .map(|row| self.parse_row(&row))?;

        Ok(new_user)
    }
}
