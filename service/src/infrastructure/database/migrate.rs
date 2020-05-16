use super::Database;
use rust_embed::RustEmbed;
use thiserror::Error;

/// The embedded migrations files to apply
#[derive(RustEmbed)]
#[folder = "migrations/"]
struct Migrations;

/// Migrate the database schema from the given connection pool to the latest version
///
/// # Parameters
/// - `database` - The database wrapper to use to connect to the database
///
/// # Returns
/// If the migration is successful then a void value is returned
///
/// # Errors
/// If the migration is unsuccessful then an error is returned indicating why
pub fn migrate_database(database: &Database) -> Result<(), MigrationError> {
    tracing::info!("Migrating database");
    let mut connection = database.checkout()?;
    let mut transaction = connection.transaction()?;

    ensure_migrations_table(&mut transaction)?;
    apply_migrations(&mut transaction)?;

    transaction.commit()?;

    Ok(())
}

/// Generate a list of all the migrations that are known.
///
/// These are loaded from the embedded SQL files, and indicate everything that can be applied
///
/// # Returns
/// The list of migration files to apply
fn list_all_migrations() -> Vec<String> {
    tracing::trace!("Listing all migrations that can be applied");
    let mut migrations: Vec<String> = Migrations::iter().map(|f| f.to_string()).collect();
    migrations.sort();
    tracing::debug!(migrations = ?migrations, "All known migrations");

    migrations
}

/// Ensure that the migrations table exists and that we've got it locked so that we can proceed with the migration
///
/// # Parameters
/// - `transaction` - The database transaction we're working in
///
/// # Returns
/// On successfully ensuring the table exists, return a void value
///
/// # Errors
/// If an error occurs executing the SQL to either create or lock the table then return an error
fn ensure_migrations_table(
    transaction: &mut postgres::Transaction<'_>,
) -> Result<(), MigrationError> {
    tracing::trace!("Ensuring the migrations table exists");
    transaction.execute(
        "CREATE TABLE IF NOT EXISTS __migrations(
            migration_file TEXT PRIMARY KEY,
            sequence SERIAL NOT NULL,
            executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
            executed_from TEXT NOT NULL DEFAULT inet_client_addr()
        )",
        &[],
    )?;

    tracing::trace!("Locking the migrations table");
    transaction.execute("LOCK TABLE __migrations IN EXCLUSIVE MODE", &[])?;

    Ok(())
}

/// Get a list of the migrations that have been previously applied
///
/// This list is loaded from the database and indicates which of the migrations have been applied before
///
/// # Parameters
/// - `transaction` - The database transaction we're working in
///
/// # Returns
/// The list of migrations that have been applied before
///
/// # Errors
/// If an error occurs executing the SQL then return an error
fn list_applied_migrations(
    transaction: &mut postgres::Transaction<'_>,
) -> Result<Vec<String>, MigrationError> {
    tracing::trace!("Listing the applied migrations");

    let migrations = transaction
        .query("SELECT migration_file FROM __migrations", &[])?
        .iter()
        .map(|row| row.get::<&str, String>("migration_file"))
        .collect::<Vec<String>>();
    tracing::debug!(migrations = ?migrations, "Migrations already applied");

    Ok(migrations)
}

/// Actually apply the migrations that are outstanding
///
/// # Parameters
/// - `transaction` - The database transaction we're working in
///
/// # Returns
/// On successfully applying the migrations, return a void value
///
/// # Errors
/// If an error occurs applying the migrations then return an error
fn apply_migrations(transaction: &mut postgres::Transaction<'_>) -> Result<(), MigrationError> {
    let all_migrations = list_all_migrations();
    let applied_migrations = list_applied_migrations(transaction)?;

    let mut count: u32 = 0;
    for migration in &all_migrations {
        if applied_migrations.contains(migration) {
            tracing::debug!(migration = ?migration, "Migration already applied");
        } else {
            tracing::debug!(migration = ?migration, "Applying migration");
            let contents = Migrations::get(migration).expect("Failed to load migration");

            transaction.batch_execute(std::str::from_utf8(&contents)?)?;
            transaction.execute(
                "INSERT INTO __migrations(migration_file) VALUES ($1)",
                &[migration],
            )?;
            count += 1;
        }
    }
    tracing::info!(count = ?count, total = ?(all_migrations.len()), "Applied migrations");

    Ok(())
}

/// Errors that can happen when migrating to the latest schema
#[derive(Error, Debug)]
pub enum MigrationError {
    /// An error occurred getting a connection from the database
    #[error("Failed to get connection from pool: {0}")]
    DatabasePool(#[from] super::DatabaseError),

    /// An error occurred executing a query against the database
    #[error("Failed to execute query against database: {0}")]
    Database(#[from] postgres::Error),

    /// An error occurred loading a file from the embedded files
    #[error("Failed to parse migration file: {0}")]
    FileParse(#[from] std::str::Utf8Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::database::TestDatabase;

    #[test]
    fn test_migrate() {
        let container = TestDatabase::default();
        let sut = Database::new(container.url);

        let result = migrate_database(&sut);
        assert!(result.is_ok());
    }
}
