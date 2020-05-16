use super::Database;
use crate::infrastructure::healthchecker::CheckHealth;
use async_trait::async_trait;
use std::boxed::Box;
use std::error::Error;

#[async_trait]
impl CheckHealth for Database {
    /// Actually check the health of the component, and return any errors that have occurred with it.
    ///
    /// This will open a connection to the database and execute a trivial query that is known to work, and
    /// will then fail if any part of this fails.
    ///
    /// # Returns
    /// In the event that the component is healthy, a void value is returned.
    ///
    /// # Errors
    /// If either a connection can not be checked out, or the trivial query fails for some reason then
    /// an error is returned to indicate that the database connection is unhealthy.
    async fn check_health(&self) -> Result<(), Box<dyn Error>> {
        let mut connection = self.checkout()?;
        connection.query("SELECT 1", &[])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::database::TestDatabase;

    #[actix_rt::test]
    async fn test_check_health() {
        let container = TestDatabase::default();
        let sut = Database::new(container.url);

        let health = sut.check_health().await;
        assert!(health.is_ok());
    }
}
