mod create;
mod errors;
mod find;
mod parse;
mod update;

use crate::infrastructure::database::Database;
pub use errors::*;

/// Repository used to access User data from the database
#[derive(Clone)]
pub(super) struct UserRepository {
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
    pub(super) fn new(database: Database) -> Self {
        Self { database }
    }
}
