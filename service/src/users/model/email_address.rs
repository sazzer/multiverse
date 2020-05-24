use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Typesafe representation of the Email Address of some user
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromSql)]
pub struct EmailAddress(String);

/// Errors that can occur when parsing an email address
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum EmailAddressParseError {
    /// The email address was blank
    #[error("The email address was blank")]
    Blank,
}

impl FromStr for EmailAddress {
    type Err = EmailAddressParseError;

    /// Parse a string into an EmailAddress object.
    ///
    /// # Parameters
    /// - `s` - The input string to parse
    ///
    /// # Returns
    /// The email address object
    ///
    /// # Errors
    /// - `EmailAddressParseError::Blank` - If the input string was blank - i.e. it was entirely whitespace
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            Err(EmailAddressParseError::Blank)
        } else {
            Ok(EmailAddress(s.to_owned()))
        }
    }
}

impl ToSql for EmailAddress {
    accepts!(TEXT, VARCHAR);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}
