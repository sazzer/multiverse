use crate::{authorization::Authorization, users::UserModel};
/// Representation of a user that we have successfully authenticated as.
///
/// This wrapper contains both the user and the authentication details
#[derive(Debug)]
pub struct AuthenticatedUserModel {
    /// The user that we authenticated as
    pub user: UserModel,
    /// The authorization details for this user
    pub authorization: Authorization,
}
