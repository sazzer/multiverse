use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The ID of a User
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, FromSql)]
pub struct UserID(Uuid);

impl ToSql for UserID {
    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }

    accepts!(UUID);
    to_sql_checked!();
}
