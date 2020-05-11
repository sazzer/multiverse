use super::UserRepository;
use crate::model::Identity;
use crate::users::model::*;

impl UserRepository {
    /// Look up the user record that has the provided Username
    ///
    /// # Parameters
    /// - `username` - The username to look up
    ///
    /// # Returns
    /// The user that was found, if there was one. `None` if one wasn't found
    #[tracing::instrument(skip(self))]
    pub async fn find_user_by_username(&self, username: &Username) -> Option<UserModel> {
        let connection = self
            .database
            .checkout()
            .await
            .expect("Failed to get database connection");
        let users = connection
            .query("SELECT * FROM users WHERE username = $1", &[username])
            .await
            .expect("Failed to query for user by username");

        users.get(0).map(|user| UserModel {
            identity: Identity {
                id: user.get("user_id"),
                version: user.get("version"),
                created: user.get("created"),
                updated: user.get("updated"),
            },
            data: UserData {
                username: user.get("username"),
                display_name: user.get("display_name"),
                email_address: user.get("email_address"),
                avatar_url: user.get("avatar_url"),
                password: user.get("password"),
            },
        })
    }
}
