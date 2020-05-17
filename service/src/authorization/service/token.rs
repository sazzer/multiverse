use crate::authorization::{AuthorizationDetails, AuthorizationService, Token};
use serde_json::json;

impl AuthorizationService {
    /// Generate an authorization token for the provided details
    ///
    /// # Parameters
    /// - `details` - The authorization details to generate a token for
    ///
    /// # Returns
    /// The opaque token that represents these authorization details
    pub(super) fn generate_token(&self, details: &AuthorizationDetails) -> Token {
        let header = json!({});
        let payload = json!({
            "iss": "tag,multiverse:2020,jwt/authorization",
            "aud": "tag,multiverse:2020,jwt/authorization",
            "sub": details.user_id,
            "exp": details.valid_until.timestamp(),
            "nbf": details.valid_from.timestamp(),
            "iat": details.valid_from.timestamp()
        });

        let jwt =
            frank_jwt::encode(header, &self.secret, &payload, frank_jwt::Algorithm::HS512).unwrap();

        Token::new(jwt)
    }
}
