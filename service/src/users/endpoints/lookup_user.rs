use super::{errors::UserProblemType, model::UserResponse};
use crate::{
    http::problem::Problem,
    users::{UserID, UsersService},
};
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
        .ok_or_else(|| Problem::new(UserProblemType::UnknownUserID, Status::NotFound))
        .map(|user| user.into())
}
