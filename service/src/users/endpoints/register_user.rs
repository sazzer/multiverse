use crate::http::problem::*;
use crate::users::*;
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::Value;

/// Actix handler to register a new user
///
/// # Parameters
/// - `user_service` - The user service to use to register the new user
/// - `body` - The incoming JSON body to work with
///
/// # Returns
/// TODO: Unknown
#[tracing::instrument(name = "POST /users", skip(_users_service, body))]
#[post("/users")]
pub async fn register_user(
    _users_service: web::Data<UsersService>,
    body: web::Json<Value>,
) -> Result<impl Responder, Problem<RegisterUserProblemType>> {
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

            Ok(HttpResponse::NoContent())
        }
        _ => {
            tracing::warn!("Validation error registering user");

            let mut problem = ValidationProblem::new(RegisterUserProblemType::Validation);

            if let Err(err) = username.map_err(|e| match e {
                UsernameParseError::Blank => GenericUserValidationProblem::Missing,
            }) {
                problem.with_field_error("username", err);
            }

            if let Err(err) = email_address.map_err(|e| match e {
                EmailAddressParseError::Blank => GenericUserValidationProblem::Missing,
            }) {
                problem.with_field_error("email_address", err);
            }

            if password.is_none() {
                problem.with_field_error("password", GenericUserValidationProblem::Missing);
            }

            Err(problem.build())
        }
    }
}

/// Problem Types that can happen when registering a user
#[derive(Debug, thiserror::Error)]
pub enum RegisterUserProblemType {
    /// The provided user details were invalid
    #[error("The provided details were invalid")]
    Validation,
}

impl ProblemType for RegisterUserProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        match self {
            RegisterUserProblemType::Validation => "tag:multiverse,2020:problems/validation_error",
        }
    }
}
