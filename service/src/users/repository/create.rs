use super::{SaveUserError, UserRepository};
use crate::users::{UserData, UserID, UserModel};
use chrono::Utc;
use uuid::Uuid;

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
    pub fn create(&self, user: UserData) -> Result<UserModel, SaveUserError> {
        let id = UserID::default();
        let now = Utc::now();
        let version = Uuid::new_v4();

        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");
        let new_user = connection.query_one("INSERT INTO users(user_id, version, created, updated, username, display_name, email_address, avatar_url, password) VALUES($1, $2, $3, $3, $4, $5, $6, $7, $8) RETURNING *", 
            &[
                &id,
                &version,
                &now,
                &user.username,
                &user.display_name,
                &user.email_address,
                &user.avatar_url,
                &user.password
            ])
            .map(|row| self.parse_row(&row))?;

        Ok(new_user)
    }
}
