use crate::authentication::AuthenticatedUserModel;
use crate::authorization::Token;
use crate::users::endpoints::model::UserResponse;
use chrono::{DateTime, Utc};
use rocket::{response, Request};
use rocket_contrib::json::Json;
use serde::Serialize;

/// API Model representing an authenticated User - that is a user and access token
#[derive(Debug, Serialize)]
pub struct AuthenticatedUserResponse {
    /// The user
    user: UserResponse,
    /// The access token
    token: AccessTokenResponse,
}

/// API Model representing an access token for a user
#[derive(Debug, Serialize)]
pub struct AccessTokenResponse {
    /// The actual token
    token: Token,
    /// The date that the token expires
    valid_until: DateTime<Utc>,
}

impl From<AuthenticatedUserModel> for AuthenticatedUserResponse {
    fn from(user: AuthenticatedUserModel) -> Self {
        Self {
            user: user.user.into(),
            token: AccessTokenResponse {
                token: user.authorization.token,
                valid_until: user.authorization.details.valid_until,
            },
        }
    }
}

impl<'r> response::Responder<'r> for AuthenticatedUserResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        response::Response::build()
            .merge(Json(self).respond_to(req).unwrap())
            .ok()
    }
}