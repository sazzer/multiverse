use super::{errors::UserProblemType, model::UserResponse};
use crate::authorization::Authorizer;
use crate::http::problem::{GenericValidation, Problem, ValidationProblem};
use crate::users::*;
use rocket::{http::Status, patch, State};
use rocket_contrib::json::Json;
use serde::Deserialize;

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
#[patch("/users/<id>", data = "<body>")]
pub fn patch_user(
    users_service: State<UsersService>,
    id: UserID,
    body: Json<PatchUserRequest>,
    authorizer: Authorizer,
) -> Result<UserResponse, Problem> {
    authorizer.authorize().same_user(&id).finish()?;

    let display_name = body.display_name();
    let email_address = body.email_address();
    let avatar_url = body.avatar_url();
    let password = body.password();

    if let (Ok(email_address), Ok(password)) = (&email_address, &password) {
        users_service
            .update_user(&id, &move |user| UserData {
                display_name: display_name.clone().unwrap_or(user.display_name),
                email_address: email_address.clone().unwrap_or(user.email_address),
                avatar_url: avatar_url.clone().or(user.avatar_url),
                password: password.clone().unwrap_or(user.password),
                ..user
            })
            .ok_or_else(|| Problem::new(UserProblemType::UnknownUserID, Status::NotFound))
            .map(|user| user.into())
    } else {
        tracing::warn!("Validation error registering user");

        let mut problem = ValidationProblem::new();

        if let Err(err) = email_address.map_err(|e| match e {
            EmailAddressParseError::Blank => GenericValidation::Missing,
        }) {
            problem.with_field_error("email_address", err);
        }

        if password.is_err() {
            problem.with_field_error("password", GenericValidation::Missing);
        }

        Err(problem.build())
    }
}

/// Incoming details representing a registration request
#[derive(Debug, Deserialize)]
pub struct PatchUserRequest {
    /// The new display name to use
    display_name: Option<String>,
    /// The new email address to use
    email_address: Option<String>,
    /// The new avatar URL to use
    avatar_url: Option<String>,
    /// The new password to use
    password: Option<Plaintext>,
}

impl PatchUserRequest {
    /// Extract the display name to use
    fn display_name(&self) -> Option<String> {
        self.display_name.clone().filter(|v| !v.trim().is_empty())
    }

    /// Extract the email address to use
    fn email_address(&self) -> Result<Option<EmailAddress>, EmailAddressParseError> {
        self.email_address.clone().map(|v| v.parse()).transpose()
    }

    /// Extract the avatar URL to use
    fn avatar_url(&self) -> Option<String> {
        self.avatar_url
            .clone()
            .filter(|v| !v.trim().is_empty())
            .map(|v| v.to_owned())
    }

    /// Extract the password to use
    fn password(&self) -> Result<Option<Password>, PasswordHashError> {
        self.password
            .clone()
            .map(Password::from_plaintext)
            .transpose()
    }
}
