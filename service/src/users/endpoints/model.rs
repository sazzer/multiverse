use crate::{
    authorization::Authorizer,
    users::{EmailAddress, UserID, UserModel, Username},
};
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
    #[serde(skip)]
    pub user_id: UserID,
    /// The Username of the User
    pub username: Username,
    /// The Display Name of the User
    pub display_name: String,
    /// The Email address of the User
    pub email_address: EmailAddress,
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
            user_id: user.identity.id,
            username: user.data.username,
            display_name: user.data.display_name,
            email_address: user.data.email_address,
            avatar_url: user.data.avatar_url,
            last_modified: user.identity.updated,
            etag: user.identity.version,
        }
    }
}

/// API Model representing a User
#[derive(Serialize)]
pub struct UserResponseModel {
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
}

impl<'r> response::Responder<'r> for UserResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let authorized = req
            .guard::<Authorizer>()
            .unwrap()
            .authorize()
            .same_user(&self.user_id)
            .finish()
            .is_ok();

        let etag = self.etag.to_string();
        let response_model = UserResponseModel {
            username: self.username,
            display_name: self.display_name,
            email_address: Some(self.email_address).filter(|_| authorized),
            avatar_url: self.avatar_url,
        };

        response::Response::build()
            .merge(Json(response_model).respond_to(req).unwrap())
            .header(CacheControl(vec![
                CacheDirective::Private,
                CacheDirective::MaxAge(3600),
            ]))
            .header(ETag(EntityTag::new(false, etag)))
            .ok()
    }
}
