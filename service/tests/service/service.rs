use super::seed::Seedable;
use multiverse_lib::{Service, Settings, TestDatabase};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use std::str::FromStr;

/// Wrapper around the service that we are testing, allowing us to interact with it as needed.
///
/// This will start up the service including a database that it works with, and allow us to send
/// queries to the database and HTTP requests to the server, including the ability to seed data into
/// the database.
pub struct TestService {
    /// The wrapper around the Docker container for the database
    _database: TestDatabase,
    /// The actual connection pool connecting to the database
    pool: Pool<PostgresConnectionManager<postgres::tls::NoTls>>,
    /// The service under test
    service: Service,
}

impl TestService {
    /// Create a Test Service ready for us to test against
    ///
    /// # Returns
    /// A constructed service ready for us to test against
    pub fn new() -> TestService {
        let _ = tracing_subscriber::fmt::try_init();

        // Build the database container
        let database = TestDatabase::default();
        let database_url = database.url.clone();

        // Open a connection pool to the database for seeding records
        let config = postgres::config::Config::from_str(&database_url).unwrap();
        let manager = PostgresConnectionManager::new(config, postgres::NoTls);

        let pool = Pool::builder()
            .connection_timeout(std::time::Duration::from_secs(10))
            .build(manager)
            .unwrap();

        // Actually build the service to test
        let settings = Settings { database_url };
        let service = multiverse_lib::Service::new(settings);

        TestService {
            _database: database,
            pool,
            service,
        }
    }

    /// Get a test client used to test the server
    ///
    /// # Returns
    /// The test client
    pub fn test_client(&self) -> rocket::local::Client {
        self.service.test_client()
    }

    /// Get a database connection to work with
    ///
    /// # Returns
    /// The database connection
    pub fn database_connection(
        &self,
    ) -> PooledConnection<PostgresConnectionManager<postgres::tls::NoTls>> {
        self.pool.get().unwrap()
    }

    /// Insert some seed data into the database
    ///
    /// # Parameters
    /// - `data` - The data to seed into the database
    ///
    /// # Returns
    /// The data that was seeded
    pub fn seed<D>(&self, data: &D)
    where
        D: Seedable,
        D: ?Sized,
    {
        tracing::debug!(data = ?data, "Inserting seed data into database");

        let mut connection = self.database_connection();
        let sql = data.sql();
        let binds = data.binds();
        let updates = connection.execute(sql, binds.as_slice()).unwrap();

        tracing::debug!(rows = ?updates, "Inserted seed data into database");
    }
}
