use super::{errors::UserProblemType, model::UserResponse};
use crate::authorization::Authorizer;
use crate::http::problem::Problem;
use crate::users::{UserID, UsersService};
use rocket::{http::Status, patch, State};

/// Actix handler to update the details of a user by their unique ID
///
/// # Parameters
/// - `user_service` - The user service to use to look up the username
/// - `id` - The ID of the user
///
/// # Returns
/// If the user is return the details.
/// If the user is not registered then an RFC-7807 problem response indicating this.
#[tracing::instrument(name = "PATCH /usernames/{id}", skip(users_service))]
#[patch("/users/<id>")]
pub fn patch_user(
    users_service: State<UsersService>,
    id: UserID,
    authorizer: Authorizer,
) -> Result<UserResponse, Problem> {
    authorizer.authorize().same_user(&id).finish()?;

    users_service
        .find_user_by_id(&id)
        .ok_or_else(|| Problem::new(UserProblemType::UnknownUserID, Status::NotFound))
        .map(|user| user.into())
}
