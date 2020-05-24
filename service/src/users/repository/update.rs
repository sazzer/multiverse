use super::{SaveUserError, UserRepository};
use crate::users::{UserData, UserID, UserModel};
use chrono::Utc;
use uuid::Uuid;

impl UserRepository {
    /// Update the given user model in the database
    ///
    /// # Parameters
    /// - `id` - The ID of the user to update
    /// - `user` - The details to update in the database
    ///
    /// # Returns
    /// The updated user
    ///
    /// # Errors
    /// Any errors that occurred updating the user
    pub fn update(&self, id: &UserID, user: UserData) -> Result<UserModel, SaveUserError> {
        let now = Utc::now();
        let version = Uuid::new_v4();

        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");
        let new_user = connection.query_opt("UPDATE users SET version = $2, updated = $3, display_name = $4, email_address = $5, avatar_url = $6, password = $7 WHERE user_id = $1 RETURNING *", 
            &[
                &id,
                &version,
                &now,
                &user.display_name,
                &user.email_address,
                &user.avatar_url,
                &user.password,
            ])?
            .map(|row| self.parse_row(&row))
            .ok_or(SaveUserError::UnknownUser)?;

        Ok(new_user)
    }
}
