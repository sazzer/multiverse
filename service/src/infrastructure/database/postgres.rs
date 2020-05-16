use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
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
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        let url = url.into();
        tracing::info!(url = ?url, "Connecting to database");

        let config = postgres::config::Config::from_str(&url).expect("Failed to parse URL");
        let manager = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

        let pool = Pool::builder()
            .connection_timeout(std::time::Duration::from_secs(10))
            .build(manager)
            .expect("Failed to create connection pool");

        pool.get().expect("Failed to check out connection");
        Self { pool }
    }

    /// Check out a connection with which we can send queries to the database
    ///
    /// # Returns
    /// A Postgres connection that can be used to communicate with the database
    pub fn checkout(
        &self,
    ) -> Result<
        PooledConnection<PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
        DatabaseError,
    > {
        self.pool
            .get()
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
