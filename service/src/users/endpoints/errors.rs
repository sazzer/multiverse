use crate::http::problem::ProblemType;

/// Problem Types that can happen when working with users
#[derive(Debug, thiserror::Error)]
pub enum UserProblemType {
    /// The user ID that was looked up was not found
    #[error("The requested user ID was unknown")]
    UnknownUserID,

    /// The username that was looked up was not found
    #[error("The requested username was unknown")]
    UnknownUsername,

    /// Changing a password without providing the correct current password
    #[error("Old Password incorrect when changing password")]
    InvalidOldPassword,
}

impl ProblemType for UserProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        match self {
            UserProblemType::UnknownUserID => "tag:multiverse,2020:users/problems/unknown_user_id",
            UserProblemType::UnknownUsername => {
                "tag:multiverse,2020:users/problems/unknown_username"
            }
            UserProblemType::InvalidOldPassword => {
                "tag:multiverse,2020:users/problems/invalid_old_password"
            }
        }
    }
}
