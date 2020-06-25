use crate::users::UserID;
use rocket::{http::RawStr, request::FromFormValue};
use std::ops::Deref;
use uuid::Uuid;

/// Representation of a link to a user
#[derive(Debug)]
pub struct UserLink(UserID);

impl UserLink {
    pub fn new(user_id: UserID) -> Self {
        Self(user_id)
    }
}

impl From<UserLink> for String {
    fn from(user_link: UserLink) -> String {
        format!("/users/{}", user_link.0)
    }
}

impl Deref for UserLink {
    type Target = UserID;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<UserLink> for UserID {
    fn from(user_link: UserLink) -> Self {
        user_link.0.clone()
    }
}

impl<'v> FromFormValue<'v> for UserLink {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        form_value
            .percent_decode()
            .map_err(|e| {
                tracing::warn!(e = ?e, value = ?form_value, "Failed to decode parameter");
                form_value
            })
            .map(|param| param.trim_start_matches("/users/").to_owned())
            .and_then(|user_id| {
                Uuid::parse_str(&user_id).map_err(|e| {
                    tracing::warn!(e = ?e, value = ?form_value, "Failed to parse User ID");
                    form_value
                })
            })
            .map(|user_id| UserID::new(user_id))
            .map(|user_id| UserLink(user_id))
    }
}
