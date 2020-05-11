use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

/// Typesafe representation of the Email Address of some user
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromSql)]
pub struct EmailAddress(String);

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
