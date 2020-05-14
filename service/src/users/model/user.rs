use super::{EmailAddress, Password, UserID, Username};
use crate::model::Model;

/// Data to represent a user record
#[derive(Debug, Clone)]
pub struct UserData {
    /// The Username of the User
    pub username: Username,
    /// The Display Name of the User
    pub display_name: String,
    /// The Email address of the User
    pub email_address: EmailAddress,
    /// The Avatar to use for the User
    pub avatar_url: Option<String>,
    /// The hashed password for the User
    pub password: Password,
}

/// Model representation of a User
pub type UserModel = Model<UserID, UserData>;
