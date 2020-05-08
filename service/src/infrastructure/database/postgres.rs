use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;

/// Wrapper around the database connection, allowing us to communicate with the database
#[derive(Clone)]
pub struct Database {
    /// The actual connection pool connecting to the database
    pool: Pool<PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
}

impl Database {
    /// Create the new database wrapper.
    ///
    /// # Parameters
    /// - `url` - The URL to use to connect to the database
    ///
    /// # Returns
    /// The wrapper connecting to the database
    pub async fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        let url = url.into();
        tracing::info!(url = ?url, "Connecting to database");

        let config = tokio_postgres::config::Config::from_str(&url).unwrap();
        let manager = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

        let pool = Pool::builder()
            .connection_timeout(std::time::Duration::from_secs(10))
            .build(manager)
            .await
            .unwrap();

        pool.get().await.unwrap();
        Self { pool }
    }

    /// Check out a connection with which we can send queries to the database
    ///
    /// # Returns
    /// A Postgres connection that can be used to communicate with the database
    pub async fn checkout(
        &self,
    ) -> Result<
        PooledConnection<'_, PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
        DatabaseError,
    > {
        self.pool
            .get()
            .await
            .map_err(|e| DatabaseError::Checkout(format!("{}", e)))
    }
}

/// Errors that can happen when working with the database
#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    /// An error occurred trying to check out a connection from the connection pool
    #[error("Error checking out connection: {0}")]
    Checkout(String),
}
