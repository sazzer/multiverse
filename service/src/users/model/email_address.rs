use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

/// Typesafe representation of the Email Address of some user
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromSql)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Construct a new EmailAddress from the given input value
    ///
    /// # Parameters
    /// - `email` - The Email address to wrap
    ///
    /// # Returns
    /// The wrapper EmailAddress
    pub fn new<S>(email: S) -> Self
    where
        S: Into<String>,
    {
        Self(email.into().trim().to_owned())
    }
}

impl ToSql for EmailAddress {
    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }

    accepts!(TEXT, VARCHAR);
    to_sql_checked!();
}
