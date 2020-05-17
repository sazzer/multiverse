use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The Identity of some model resource
///
/// # Types
/// - `<ID>` - The type to use for the actual ID
#[derive(Debug)]
pub struct Identity<ID> {
    /// The actual ID of the resource
    pub id: ID,
    /// The version tag of the resource
    pub version: Uuid,
    /// When the resource was first created
    pub created: DateTime<Utc>,
    /// When the resource was last updated
    pub updated: DateTime<Utc>,
}

/// The representation of some model resource
///
/// # Types
/// - `<ID>` - The type to use for the ID of the resource
/// - `<DATA>` - The type to use for the actual data
#[derive(Debug)]
pub struct Model<ID, DATA> {
    pub identity: Identity<ID>,
    pub data: DATA,
}

impl<ID> Default for Identity<ID>
where
    ID: Default,
{
    /// Construct a default instance of the Identity.
    /// Generate the appropriate default value for `id`, a totally random UUID for `version` and then the
    /// current system time for `created` and `updated`
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: ID::default(),
            version: Uuid::new_v4(),
            created: now,
            updated: now,
        }
    }
}
