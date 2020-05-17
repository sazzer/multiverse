use crate::authorization::{Authorization, AuthorizationService};
use crate::users::UserModel;
use chrono::Utc;

impl AuthorizationService {
    /// Generate an Authorization record for the given user
    ///
    /// # Parameters
    /// - `user` - The user to generate the authorization record for
    ///
    /// # Returns
    /// The authorization record
    pub fn generate_authorization(&self, user: &UserModel) -> Authorization {
        let start = Utc::now();
        let end = start + self.duration;

        Authorization {
            user_id: user.identity.id.clone(),
            valid_from: start,
            valid_until: end,
        }
    }
}
