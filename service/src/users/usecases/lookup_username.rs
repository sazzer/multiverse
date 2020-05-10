use crate::users::{Username, UsersService};

impl UsersService {
    /// Look up a Username to see if it's known to the system
    ///
    /// # Parameters
    /// - `username` - The username to look up
    ///
    /// # Returns
    /// True if the username exists. False if not
    pub async fn lookup_username(&self, username: &Username) -> bool {
        self.repository
            .find_user_by_username(username)
            .await
            .is_some()
    }
}
