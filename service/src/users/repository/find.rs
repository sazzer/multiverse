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
        let connection = self.database.checkout().await.unwrap();
        let users = connection
            .query("SELECT * FROM users WHERE username = $1", &[username])
            .await
            .unwrap();

        users.get(0).map(|user| UserModel {
            identity: Identity {
                id: user.try_get("user_id").unwrap(),
                version: user.try_get("version").unwrap(),
                created: user.try_get("created").unwrap(),
                updated: user.try_get("updated").unwrap(),
            },
            data: UserData {
                username: user.try_get("username").unwrap(),
                display_name: user.try_get("display_name").unwrap(),
                email_address: user.try_get("email_address").unwrap(),
                avatar_url: user.try_get("avatar_url").unwrap(),
            },
        })
    }
}
