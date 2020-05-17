use crate::authorization::{AuthorizationService, Token};
use crate::users::UserID;
use chrono::{DateTime, Utc};

impl AuthorizationService {
    pub(super) fn generate_token(
        &self,
        user: &UserID,
        valid_from: &DateTime<Utc>,
        valid_until: &DateTime<Utc>,
    ) -> Token {
        Token::new("Hello")
    }
}
