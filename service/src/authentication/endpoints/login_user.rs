use super::model::AuthenticatedUserResponse;
use crate::http::problem::*;
use crate::{authentication::AuthenticationService, users::*};
use rocket::{post, State};
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
#[tracing::instrument(name = "POST /login", skip(authentication_service, body))]
#[post("/login", data = "<body>")]
pub fn login_user(
    authentication_service: State<AuthenticationService>,
    body: Json<LoginRequest>,
) -> Result<AuthenticatedUserResponse, Problem> {
    let username = body.username();
    let password = body.password();

    match (&username) {
        (Ok(username)) => todo!(),
        _ => {
            tracing::warn!("Validation error authenticating user");

            todo!()
        }
    }
}

/// Incoming details representing a login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// The username to log in as
    username: Option<String>,
    /// The password to log in with
    password: Option<String>,
}

impl LoginRequest {
    /// Extract the username to use
    fn username(&self) -> Result<Username, UsernameParseError> {
        self.username
            .clone()
            .unwrap_or("".to_owned())
            .parse::<Username>()
    }

    /// Extract the password to use
    fn password(&self) -> String {
        self.password
            .clone()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_default()
    }
}
