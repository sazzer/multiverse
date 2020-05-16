use crate::http::problem::{Problem, ProblemType};
use crate::users::{Username, UsersService};
use rocket::{get, http::Status, State};

/// Actix handler to see if a username is already registered or not
///
/// # Parameters
/// - `user_service` - The user service to use to look up the username
/// - `path` - The details of the parameters from the URL
///
/// # Returns
/// If the username is known then an empty response.
/// If the username is not registered then an RFC-7807 problem response indicating this.
#[tracing::instrument(name = "GET /usernames/{username}", skip(users_service))]
#[get("/usernames/<username>")]
pub fn lookup_username(
    users_service: State<UsersService>,
    username: Username,
) -> Result<Status, Problem> {
    let found = users_service.lookup_username(&username);

    tracing::debug!(found = ?found, username = ?username, "Looking up username");
    if found {
        Ok(Status::NoContent)
    } else {
        Err(Problem::new(
            LookupUsernameProblemType::UnknownUsername,
            Status::NotFound,
        ))
    }
}

/// Problem Types that can happen when looking up a username
#[derive(Debug, thiserror::Error)]
pub enum LookupUsernameProblemType {
    /// The username that was looked up was not found
    #[error("The requested username was unknown")]
    UnknownUsername,
}

impl ProblemType for LookupUsernameProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        match self {
            LookupUsernameProblemType::UnknownUsername => {
                "tag:multiverse,2020:users/problems/unknown_username"
            }
        }
    }
}
