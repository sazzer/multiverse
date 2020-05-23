use crate::users::{UserData, UserID, UserModel, UsersService};

impl UsersService {
    /// Find a user by it's ID and update the data associated with it
    ///
    /// # Parameters
    /// - `id` - The User ID to look up
    ///
    /// # Returns
    /// The user if it was found. `None` if not.
    pub fn update_user(
        &self,
        id: &UserID,
        updater: &dyn Fn(UserData) -> UserData,
    ) -> Option<UserModel> {
        let user = self.repository.find_user_by_id(id).unwrap();

        let updated_user = updater(user.data);

        self.repository.update(id, updated_user).ok()
    }
}
