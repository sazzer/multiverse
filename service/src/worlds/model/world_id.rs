use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use rocket::{http::RawStr, request};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// The ID of a World
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, FromSql)]
pub struct WorldID(Uuid);

impl WorldID {
    /// Wrap a UUID as a World ID object
    ///
    /// # Parameters
    /// - `uuid` - The UUID to wrap
    ///
    /// # The World ID
    pub fn new(uuid: Uuid) -> Self {
        WorldID(uuid)
    }
}

impl Default for WorldID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl ToSql for WorldID {
    accepts!(UUID);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}

impl<'r> request::FromParam<'r> for WorldID {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        param
            .percent_decode()
            .map(|cow| cow.into_owned())
            .map_err(|_| param)
            .and_then(|world_id| Uuid::parse_str(&world_id).map_err(|_| param))
            .map(|world_id| WorldID::new(world_id))
    }
}

impl Display for WorldID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
