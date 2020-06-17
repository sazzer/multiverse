use crate::infrastructure::database::Database;

/// Repository used to access World data from the database
#[derive(Clone)]
pub(super) struct WorldRepository {
    /// The database with which to access user records
    database: Database,
}

impl WorldRepository {
    /// Create a new World Repository to work with
    ///
    /// # Parameters
    /// - `database` - The database connection to use
    ///
    /// # Returns
    /// The World Repository to use
    pub(super) fn new(database: Database) -> Self {
        Self { database }
    }
}
