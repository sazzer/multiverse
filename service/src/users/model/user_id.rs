use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use rocket::{http::RawStr, request};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// The ID of a User
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, FromSql, Eq, Hash)]
pub struct UserID(Uuid);

impl UserID {
    /// Wrap a UUID as a User ID object
    ///
    /// # Parameters
    /// - `uuid` - The UUID to wrap
    ///
    /// # The User ID
    pub fn new(uuid: Uuid) -> Self {
        UserID(uuid)
    }
}

impl Default for UserID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl ToSql for UserID {
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

impl<'r> request::FromParam<'r> for UserID {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        param
            .percent_decode()
            .map(|cow| cow.into_owned())
            .map_err(|_| param)
            .and_then(|user_id| Uuid::parse_str(&user_id).map_err(|_| param))
            .map(|user_id| UserID::new(user_id))
    }
}

impl Display for UserID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
