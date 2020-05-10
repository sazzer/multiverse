use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The ID of a User
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct UserID(Uuid);

impl UserID {
    /// Construct a new UserID from the given input value
    ///
    /// # Parameters
    /// - `user_id` - The ID to wrap
    ///
    /// # Returns
    /// The wrapper UserID
    pub fn new<S>(user_id: S) -> Self
    where
        S: Into<Uuid>,
    {
        Self(user_id.into())
    }
}
