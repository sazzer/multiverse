use super::Token;
use crate::users::UserID;
use chrono::{DateTime, Utc};

/// The actual authorization details that represent a user session
#[derive(Debug, Clone)]
pub struct Authorization {
    /// The use that the authorization details are for
    pub user_id: UserID,
    /// When the authorization details are valid from
    pub valid_from: DateTime<Utc>,
    /// When the authorization details are valid until
    pub valid_until: DateTime<Utc>,
    /// The actual authorization token, encapsulating all of the above
    pub token: Token,
}
