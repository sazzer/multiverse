use crate::users::{EmailAddress, UserModel, Username};
use serde::Serialize;

/// API Model representing a User
#[derive(Debug, Serialize)]
pub struct User {
    /// The Username of the User
    pub username: Username,
    /// The Display Name of the User
    pub display_name: String,
    /// The Email address of the User
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<EmailAddress>,
    /// The Avatar to use for the User
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

impl From<UserModel> for User {
    fn from(user: UserModel) -> Self {
        Self {
            username: user.data.username,
            display_name: user.data.display_name,
            email_address: Some(user.data.email_address),
            avatar_url: user.data.avatar_url,
        }
    }
}
