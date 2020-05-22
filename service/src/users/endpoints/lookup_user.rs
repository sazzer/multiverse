use super::model::UserResponse;
use crate::http::problem::{Problem, ProblemType};
use crate::users::{UserID, UsersService};
use rocket::{get, http::Status, State};

/// Actix handler to get the details of a user by their unique ID
///
/// # Parameters
/// - `user_service` - The user service to use to look up the username
/// - `id` - The ID of the user
///
/// # Returns
/// If the user is return the details.
/// If the user is not registered then an RFC-7807 problem response indicating this.
#[tracing::instrument(name = "GET /usernames/{id}", skip(users_service))]
#[get("/users/<id>")]
pub fn lookup_user(
    users_service: State<UsersService>,
    id: UserID,
) -> Result<UserResponse, Problem> {
    users_service
        .find_user_by_id(&id)
        .ok_or_else(|| Problem::new(LookupUserProblemType::UnknownUserID, Status::NotFound))
        .map(|user| user.into())
}

/// Problem Types that can happen when looking up a User ID
#[derive(Debug, thiserror::Error)]
pub enum LookupUserProblemType {
    /// The user ID that was looked up was not found
    #[error("The requested user ID was unknown")]
    UnknownUserID,
}

impl ProblemType for LookupUserProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        match self {
            LookupUserProblemType::UnknownUserID => {
                "tag:multiverse,2020:users/problems/unknown_user_id"
            }
        }
    }
}
