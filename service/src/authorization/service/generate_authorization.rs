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
        let valid_from = Utc::now();
        let valid_until = valid_from + self.duration;

        let token = self.generate_token(&user.identity.id, &valid_from, &valid_until);

        Authorization {
            user_id: user.identity.id.clone(),
            valid_from,
            valid_until,
            token,
        }
    }
}
