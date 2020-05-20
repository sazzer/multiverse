use crate::users::{EmailAddress, UserModel, Username};
use chrono::{DateTime, Utc};
use rocket::{
    http::hyper::header::{CacheControl, CacheDirective, ETag, EntityTag},
    response, Request,
};
use rocket_contrib::json::Json;
use serde::Serialize;
use uuid::Uuid;

/// API Model representing a User
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// The Username of the User
    pub username: Username,
    /// The Display Name of the User
    pub display_name: String,
    /// The Email address of the User
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<EmailAddress>,
    /// The Avatar to use for the User
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// The last modified date of the response
    #[serde(skip)]
    pub last_modified: DateTime<Utc>,
    /// The version of the resource, to use as an ETag
    #[serde(skip)]
    pub etag: Uuid,
}

impl From<UserModel> for UserResponse {
    fn from(user: UserModel) -> Self {
        Self {
            username: user.data.username,
            display_name: user.data.display_name,
            email_address: Some(user.data.email_address),
            avatar_url: user.data.avatar_url,
            last_modified: user.identity.updated,
            etag: user.identity.version,
        }
    }
}

impl<'r> response::Responder<'r> for UserResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let etag = self.etag.to_string();

        response::Response::build()
            .merge(Json(self).respond_to(req).unwrap())
            .header(CacheControl(vec![
                CacheDirective::Private,
                CacheDirective::MaxAge(3600),
            ]))
            .header(ETag(EntityTag::new(false, etag)))
            .ok()
    }
}
