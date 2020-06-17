use crate::users::UserID;

/// Representation of a link to a user
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
