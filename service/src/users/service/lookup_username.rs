use crate::users::{Username, UsersService};

impl UsersService {
    /// Look up a Username to see if it's known to the system
    ///
    /// # Parameters
    /// - `username` - The username to look up
    ///
    /// # Returns
    /// True if the username exists. False if not
    pub fn lookup_username(&self, username: &Username) -> bool {
        self.find_user_by_username(username).is_some()
    }
}
