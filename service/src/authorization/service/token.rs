use crate::authorization::{AuthorizationDetails, AuthorizationService, Token};
use crate::users::UserID;
use chrono::{offset::TimeZone, Utc};
use serde_json::json;
use uuid::Uuid;

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

    /// Parse a token and return the authorization details that it represents.
    ///
    /// # Parameters
    /// - `token` - The tokent to parse
    ///
    /// # Returns
    /// The authorization details
    ///
    /// # Errors
    /// If the authorization details are somehow invalid.
    /// This includes if they are malformed, expired or not yet valid.
    pub fn parse_token(&self, token: &Token) -> Result<AuthorizationDetails, ParseTokenError> {
        let (_, decoded) = frank_jwt::decode(
            token.as_ref(),
            &self.secret,
            frank_jwt::Algorithm::HS512,
            &frank_jwt::ValidationOptions::new(),
        )
        .map_err(|e| {
            tracing::warn!(e = ?e, token = ?token, "Failed to decode token");
            match e {
                frank_jwt::Error::SignatureExpired => ParseTokenError::ExpiredToken,
                _ => ParseTokenError::MalformedToken,
            }
        })?;

        tracing::debug!(decoded = ?decoded, "Decoded token");
        decoded
            .get("iss")
            .and_then(|v| v.as_str())
            .filter(|v| v == &"tag,multiverse:2020,jwt/authorization")
            .ok_or(ParseTokenError::InvalidIssuer)?;
        decoded
            .get("aud")
            .and_then(|v| v.as_str())
            .filter(|v| v == &"tag,multiverse:2020,jwt/authorization")
            .ok_or(ParseTokenError::InvalidAudience)?;

        let user_id = decoded
            .get("sub")
            .and_then(|v| v.as_str())
            .ok_or(ParseTokenError::InvalidSubject)
            .and_then(|v| Uuid::parse_str(v).map_err(|_| ParseTokenError::InvalidSubject))
            .map(|v| UserID::new(v))?;

        let valid_until = decoded
            .get("exp")
            .and_then(|v| v.as_i64())
            .map(|timestamp| Utc.timestamp(timestamp, 0))
            .ok_or(ParseTokenError::InvalidExpiry)?;

        let valid_from = decoded
            .get("iat")
            .and_then(|v| v.as_i64())
            .map(|timestamp| Utc.timestamp(timestamp, 0))
            .ok_or(ParseTokenError::InvalidIssuedAt)?;

        Ok(AuthorizationDetails {
            user_id,
            valid_from,
            valid_until,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseTokenError {
    #[error("The token was malformed")]
    MalformedToken,

    #[error("The issuer was invalid")]
    InvalidIssuer,

    #[error("The subject was invalid")]
    InvalidSubject,

    #[error("The audience was invalid")]
    InvalidAudience,

    #[error("The expiry date was invalid")]
    InvalidExpiry,

    #[error("The issued at date was invalid")]
    InvalidIssuedAt,

    #[error("The token has expired")]
    ExpiredToken,
}
