use super::model::AuthenticatedUserResponse;
use crate::http::problem::*;
use crate::{
    authentication::{AuthenticationService, RegisterError},
    users::*,
};
use rocket::{http::Status, post, State};
use rocket_contrib::json::Json;
use serde::Deserialize;

/// Actix handler to register a new user
///
/// # Parameters
/// - `authentication_service` - The authentication service to use to register the new user
/// - `body` - The incoming JSON body to work with
///
/// # Returns
/// The details of the newly authenticated user
///
/// # Error
/// An RFC-7807 Problem if the incoming details are invalid, or the registration fails for any reason
#[tracing::instrument(name = "POST /register", skip(authentication_service, body))]
#[post("/register", data = "<body>")]
pub fn register_user(
    authentication_service: State<AuthenticationService>,
    body: Json<RegistrationRequest>,
) -> Result<AuthenticatedUserResponse, Problem> {
    let username = body.username();
    let display_name = body.display_name();
    let email_address = body.email_address();
    let avatar_url = body.avatar_url();
    let password = body.password();

    tracing::debug!(username = ?username, display_name = ?display_name, email_address = ?email_address, avatar_url = ?avatar_url, password = ?password, "Registering new user");

    match (&username, &email_address, &password) {
        (Ok(username), Ok(email_address), Some(password)) => {
            let user = UserData {
                username: username.clone(),
                display_name,
                email_address: email_address.clone(),
                avatar_url,
                password: password.clone(),
            };
            tracing::debug!(user = ?user, "Registering user");
            let authenticated_user = authentication_service.register_user(user)?;

            tracing::debug!(authenticated_user = ?authenticated_user, "Registered user");

            Ok(authenticated_user.into())
        }
        _ => {
            tracing::warn!("Validation error registering user");

            let mut problem = ValidationProblem::new();

            if let Err(err) = username.map_err(|e| match e {
                UsernameParseError::Blank => GenericValidation::Missing,
            }) {
                problem.with_field_error("username", err);
            }

            if let Err(err) = email_address.map_err(|e| match e {
                EmailAddressParseError::Blank => GenericValidation::Missing,
            }) {
                problem.with_field_error("email_address", err);
            }

            if password.is_none() {
                problem.with_field_error("password", GenericValidation::Missing);
            }

            Err(problem.build())
        }
    }
}

/// Incoming details representing a registration request
#[derive(Debug, Deserialize)]
pub struct RegistrationRequest {
    /// The username to register as
    username: Option<String>,
    /// The display name to register with
    display_name: Option<String>,
    /// The email address to register with
    email_address: Option<String>,
    /// The avatar URL to register with
    avatar_url: Option<String>,
    /// The password to register with
    password: Option<String>,
}

impl RegistrationRequest {
    /// Extract the username to use
    fn username(&self) -> Result<Username, UsernameParseError> {
        self.username
            .clone()
            .unwrap_or("".to_owned())
            .parse::<Username>()
    }

    /// Extract the display name to use
    fn display_name(&self) -> String {
        self.display_name
            .clone()
            .filter(|v| !v.trim().is_empty())
            .or_else(|| self.username.clone())
            .unwrap_or_else(|| "".to_owned())
    }

    /// Extract the email address to use
    fn email_address(&self) -> Result<EmailAddress, EmailAddressParseError> {
        self.email_address
            .clone()
            .unwrap_or("".to_owned())
            .parse::<EmailAddress>()
    }

    /// Extract the avatar URL to use
    fn avatar_url(&self) -> Option<String> {
        self.avatar_url
            .clone()
            .filter(|v| !v.trim().is_empty())
            .map(|v| v.to_owned())
    }

    /// Extract the password to use
    fn password(&self) -> Option<Password> {
        self.password
            .clone()
            .filter(|v| !v.trim().is_empty())
            .map(Password::from_plaintext)
    }
}

/// Problem Types that can happen when registering a user
#[derive(Debug, thiserror::Error)]
pub enum RegisterUserProblemType {
    /// An unknown error occurred
    #[error("An unknown error occurred")]
    UnknownError,

    #[error("The username is already registered")]
    DuplicateUsername,
}

impl ProblemType for RegisterUserProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        match self {
            RegisterUserProblemType::UnknownError => {
                "tag:multiverse,2020:users/problems/unknown_error"
            }
            RegisterUserProblemType::DuplicateUsername => {
                "tag:multiverse,2020:users/problems/duplicate_username"
            }
        }
    }
}

impl From<RegisterError> for Problem {
    fn from(e: RegisterError) -> Self {
        match e {
            RegisterError::DuplicateUsername => Problem::new(
                RegisterUserProblemType::DuplicateUsername,
                Status::UnprocessableEntity,
            ),
            _ => Problem::new(
                RegisterUserProblemType::UnknownError,
                Status::InternalServerError,
            ),
        }
    }
}
