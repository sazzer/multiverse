use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use rocket::{http::RawStr, request};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Typesafe representation of the Username of some user
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromSql)]
pub struct Username(String);

/// Errors that can occur when parsing a username
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum UsernameParseError {
    /// The username was blank
    #[error("The username was blank")]
    Blank,
}

impl FromStr for Username {
    type Err = UsernameParseError;

    /// Parse a string into a Username object.
    ///
    /// # Parameters
    /// - `s` - The input string to parse
    ///
    /// # Returns
    /// The username object
    ///
    /// # Errors
    /// - `UsernameParseError::Blank` - If the input string was blank - i.e. it was entirely whitespace
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            Err(UsernameParseError::Blank)
        } else {
            Ok(Username(s.to_owned()))
        }
    }
}

impl ToSql for Username {
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

impl<'r> request::FromParam<'r> for Username {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        param
            .percent_decode()
            .map(|cow| cow.into_owned())
            .map_err(|_| param)
            .and_then(|username| Username::from_str(&username).map_err(|_| param))
    }
}
