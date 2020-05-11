use crate::infrastructure::database::Database;

mod find;

/// Repository used to access User data from the database
#[derive(Clone)]
pub struct UserRepository {
    /// The database with which to access user records
    database: Database,
}

impl UserRepository {
    /// Create a new User Repository to work with
    ///
    /// # Parameters
    /// - `database` - The database connection to use
    ///
    /// # Returns
    /// The User Repository to use
    pub fn new(database: Database) -> Self {
        Self { database: database }
    }
}