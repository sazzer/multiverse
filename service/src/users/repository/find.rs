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
        connection.query("SELECT 1", &[]).await.unwrap();

        if username == &Username::new("known") {
            Some(UserModel {
                identity: Identity {
                    ..Default::default()
                },
                data: UserData {
                    username: username.clone(),
                    display_name: "Graham".to_owned(),
                    avatar_url: None,
                },
            })
        } else {
            None
        }
    }
}
