use super::model::AuthenticatedUserResponse;
use crate::{authentication::AuthenticationService, http::problem::*, users::*};
use rocket::{http::Status, post, State};
use rocket_contrib::json::Json;
use serde::Deserialize;

/// Actix handler to log in as an existing user
///
/// # Parameters
/// - `authentication_service` - The authentication service to use to log in as the user
/// - `body` - The incoming JSON body to work with
///
/// # Returns
/// The details of the newly authenticated user
///
/// # Error
/// An RFC-7807 Problem if the incoming details are invalid, or the registration fails for any reason
#[tracing::instrument(name = "POST /login", skip(authentication_service))]
#[post("/login", data = "<body>")]
pub fn login_user(
    authentication_service: State<AuthenticationService>,
    body: Json<LoginRequest>,
) -> Result<AuthenticatedUserResponse, Problem> {
    match (&body.username, &body.password) {
        (Some(username), Some(password)) => authentication_service
            .authenticate_user(username.clone(), password.clone())
            .map(|authenticated_user| {
                tracing::debug!(authenticated_user = ?authenticated_user, "Authenticated user");
                authenticated_user.into()
            })
            .map_err(|e| {
                tracing::warn!("Failed to authenticate user: {:?}", e);
                Problem::new(LoginProblemType {}, Status::Unauthorized)
            }),
        _ => Err(Problem::new(LoginProblemType {}, Status::Unauthorized)),
    }
}

/// Incoming details representing a login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// The username to log in as
    pub username: Option<Username>,
    /// The password to log in with
    pub password: Option<Plaintext>,
}

#[derive(Debug)]
struct LoginProblemType {}

impl std::fmt::Display for LoginProblemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Username or Password")
    }
}

impl ProblemType for LoginProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        "tag:multiverse,2020:users/problems/authentication_error"
    }
}
