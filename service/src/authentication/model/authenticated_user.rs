use crate::users::UserModel;

/// Representation of a user that we have successfully authenticated as.
///
/// This wrapper contains both the user and the authentication details
pub struct AuthenticatedUser {
    /// The user that we authenticated as
    pub user: UserModel,
}
