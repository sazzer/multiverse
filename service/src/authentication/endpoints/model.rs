use crate::{
    authentication::AuthenticatedUserModel,
    authorization::Token,
    http::link::{Link, LinkRel, Links},
    users::UserID,
};
use chrono::{DateTime, Utc};
use rocket::{response, Request};
use rocket_contrib::json::Json;
use serde::Serialize;

/// API Model representing an authenticated User - that is a user and access token
#[derive(Debug, Serialize)]
pub struct AuthenticatedUserResponse {
    /// The actual token
    token: Token,
    /// The date that the token expires
    valid_until: DateTime<Utc>,
    /// The ID of the user that authenticated
    user_id: UserID,
    /// The display name of the user that authenticated
    display_name: String,
}

impl From<AuthenticatedUserModel> for AuthenticatedUserResponse {
    fn from(user: AuthenticatedUserModel) -> Self {
        Self {
            token: user.authorization.token,
            valid_until: user.authorization.details.valid_until,
            user_id: user.user.identity.id,
            display_name: user.user.data.display_name,
        }
    }
}

impl<'r> response::Responder<'r> for AuthenticatedUserResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        response::Response::build()
            .merge(Json(&self).respond_to(req).unwrap())
            .header(Links(vec![Link::new(
                format!("/users/{}", self.user_id),
                LinkRel::RELATED,
            )]))
            .ok()
    }
}
