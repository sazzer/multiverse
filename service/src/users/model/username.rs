use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

/// Typesafe representation of the Username of some user
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromSql)]
pub struct Username(String);

impl Username {
    /// Construct a new Username from the given input value
    ///
    /// # Parameters
    /// - `username` - The username to wrap
    ///
    /// # Returns
    /// The wrapper Username
    pub fn new<S>(username: S) -> Self
    where
        S: Into<String>,
    {
        Self(username.into().trim().to_owned())
    }
}

impl ToSql for Username {
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
