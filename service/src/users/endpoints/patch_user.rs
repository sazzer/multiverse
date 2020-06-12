use super::{errors::UserProblemType, model::UserResponse};
use crate::{
    authorization::Authorizer,
    http::{
        patch::Patch,
        problem::{GenericValidation, Problem, ValidationProblem},
    },
    users::*,
};
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

    // Display Name: Missing = No Change, Null = Validation Error, Value = Update
    let display_name = body.display_name().ok_not_null(GenericValidation::Missing);

    // Email Address: Missing = No Change, Null = Validation Error, Value = Update
    let email_address = body
        .email_address()
        .map_err(|_| GenericValidation::Missing)
        .and_then(|email| email.ok_not_null(GenericValidation::Missing));

    // Avatar URL: Missing = No Change, Null = Clear, Value = Update
    let avatar_url = body.avatar_url();

    // Password: Missing = No Change, Null = Validation Error, Value = Update
    let password = body
        .password()
        .map_err(|_| GenericValidation::Missing)
        .and_then(|password| password.ok_not_null(GenericValidation::Missing));

    if let (Ok(display_name), Ok(email_address), Ok(password)) =
        (&display_name, &email_address, &password)
    {
        tracing::info!("Updating user");
        if let (Some(_), None) = (&password, &body.old_password) {
            // A new password was provided but an old one wasn't
            tracing::warn!("Attempt to change password when an old one wasn't provided");
            Err(Problem::new(
                UserProblemType::InvalidOldPassword,
                Status::UnprocessableEntity,
            ))?
        }

        users_service
            .update_user::<Problem>(&id, &move |user| {
                if let Some(old_password) = &body.old_password {
                    if (user.password != old_password.clone()) {
                        Err(Problem::new(
                            UserProblemType::InvalidOldPassword,
                            Status::UnprocessableEntity,
                        ))?
                    }
                }

                let new_user = UserData {
                    display_name: display_name.clone().unwrap_or(user.display_name),
                    email_address: email_address.clone().unwrap_or(user.email_address),
                    avatar_url: match avatar_url.clone() {
                        Patch::Value(v) => Some(v),
                        Patch::Null => None,
                        Patch::Missing => user.avatar_url,
                    },
                    password: password.clone().unwrap_or(user.password),
                    ..user
                };
                Ok(new_user)
            })
            .map_err(|e| match e {
                UpdateError::ClientError(e) => e,
                _ => Problem::new(UserProblemType::UnknownUserID, Status::NotFound),
            })
            .map(|user| user.into())
    } else {
        tracing::warn!("Validation error updating user");

        let mut problem = ValidationProblem::new();

        if let Err(err) = display_name {
            problem.with_field_error("display_name", err);
        }
        if let Err(err) = email_address {
            problem.with_field_error("email_address", err);
        }
        if let Err(err) = password {
            problem.with_field_error("password", err);
        }

        Err(problem.build())
    }
}

/// Incoming details representing a registration request
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct PatchUserRequest {
    /// The new display name to use
    display_name: Patch<String>,
    /// The new email address to use
    email_address: Patch<String>,
    /// The new avatar URL to use
    avatar_url: Patch<String>,
    /// The new password to use
    password: Patch<Plaintext>,
    /// The old password to compare against if changing it
    old_password: Option<Plaintext>,
}

impl PatchUserRequest {
    /// Extract the display name to use
    fn display_name(&self) -> Patch<String> {
        self.display_name
            .clone()
            .filter_null(|v| !v.trim().is_empty())
    }

    /// Extract the email address to use
    fn email_address(&self) -> Result<Patch<EmailAddress>, EmailAddressParseError> {
        self.email_address.clone().map(|v| v.parse()).transpose()
    }

    /// Extract the avatar URL to use
    fn avatar_url(&self) -> Patch<String> {
        self.avatar_url
            .clone()
            .filter_null(|v| !v.trim().is_empty())
            .map(|v| v.to_owned())
    }

    /// Extract the password to use
    fn password(&self) -> Result<Patch<Password>, PasswordHashError> {
        self.password
            .clone()
            .map(Password::from_plaintext)
            .transpose()
    }
}
