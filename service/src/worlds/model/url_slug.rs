use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use rocket::{http::RawStr, request};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Typesafe representation of the URL Slug of some world
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromSql)]
pub struct UrlSlug(String);

/// Errors that can occur when parsing a URL Slug
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum UrlSlugParseError {
    /// The URL Slug was blank
    #[error("The URL Slug was blank")]
    Blank,
}

impl FromStr for UrlSlug {
    type Err = UrlSlugParseError;

    /// Parse a string into a UrlSlug object.
    ///
    /// # Parameters
    /// - `s` - The input string to parse
    ///
    /// # Returns
    /// The URL Slug object
    ///
    /// # Errors
    /// - `UrlSlugParseError::Blank` - If the input string was blank - i.e. it was entirely whitespace
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            Err(UrlSlugParseError::Blank)
        } else {
            Ok(UrlSlug(s.to_owned()))
        }
    }
}

impl ToSql for UrlSlug {
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

impl<'r> request::FromParam<'r> for UrlSlug {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        param
            .percent_decode()
            .map(|cow| cow.into_owned())
            .map_err(|_| param)
            .and_then(|url_slug| UrlSlug::from_str(&url_slug).map_err(|_| param))
    }
}
