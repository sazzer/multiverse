use crate::http::problem::*;
use crate::{
    authentication::{AuthenticationService, RegisterError},
    users::*,
};
use actix_web::{http::StatusCode, post, web, HttpResponse, Responder};
use serde_json::Value;

/// Actix handler to register a new user
///
/// # Parameters
/// - `authentication_service` - The authentication service to use to register the new user
/// - `body` - The incoming JSON body to work with
///
/// # Returns
/// TODO: Unknown
#[tracing::instrument(name = "POST /register", skip(authentication_service, body))]
#[post("/register")]
pub async fn register_user(
    authentication_service: web::Data<AuthenticationService>,
    body: web::Json<Value>,
) -> Result<impl Responder, Problem> {
    let username = body
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .parse::<Username>();
    let display_name = body
        .get("display_name")
        .and_then(|v| v.as_str())
        .filter(|v| !v.trim().is_empty())
        .or_else(|| body.get("username").and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_owned();
    let email_address = body
        .get("email_address")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .parse::<EmailAddress>();
    let avatar_url = body
        .get("avatar_url")
        .and_then(|v| v.as_str())
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.to_owned());
    let password = body
        .get("password")
        .and_then(|v| v.as_str())
        .filter(|v| !v.trim().is_empty())
        .map(Password::from_plaintext);

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
            tracing::info!(user = ?user, "Registering user");
            authentication_service.register_user(user)?;

            Ok(HttpResponse::NoContent())
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
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            _ => Problem::new(
                RegisterUserProblemType::UnknownError,
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
