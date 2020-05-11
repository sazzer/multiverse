use super::seed::Seedable;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use multiverse_lib::{Service, Settings, TestDatabase, TestResponse};
use std::str::FromStr;
use std::sync::Arc;

/// Wrapper around the service that we are testing, allowing us to interact with it as needed.
///
/// This will start up the service including a database that it works with, and allow us to send
/// queries to the database and HTTP requests to the server, including the ability to seed data into
/// the database.
pub struct TestService {
    /// The wrapper around the Docker container for the database
    _database: TestDatabase,
    /// The actual connection pool connecting to the database
    pool: Pool<PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
    /// The service under test
    service: Service,
}

impl TestService {
    /// Create a Test Service ready for us to test against
    ///
    /// # Returns
    /// A constructed service ready for us to test against
    pub async fn new() -> TestService {
        // Build the database container
        let database = TestDatabase::default();
        let database_url = database.url.clone();

        // Open a connection pool to the database for seeding records
        let config = tokio_postgres::config::Config::from_str(&database_url).unwrap();
        let manager = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

        let pool = Pool::builder()
            .connection_timeout(std::time::Duration::from_secs(10))
            .build(manager)
            .await
            .unwrap();

        // Actually build the service to test
        let settings = Settings {
            database_url: database_url.clone(),
        };
        let service = multiverse_lib::Service::new(settings).await;

        TestService {
            _database: database,
            pool,
            service,
        }
    }

    /// Send an HTTP Request in to the service and return the response
    ///
    /// # Parameters
    /// - `request` - The request to send to the service
    ///
    /// # Returns
    /// The HTTP Response
    pub async fn request(&self, request: actix_http::Request) -> TestResponse {
        self.service.test_request(request).await
    }

    /// Insert some seed data into the database
    ///
    /// # Parameters
    /// - `data` - The data to seed into the database
    pub async fn seed(&self, data: Arc<dyn Seedable>) {
        tracing::debug!("Inserting seed data into database");

        let connection = self.pool.get().await.unwrap();
        let sql = data.sql();
        let binds = data.binds();
        let updates = connection.execute(sql, binds.as_slice()).await.unwrap();

        tracing::debug!(rows = ?updates, "Inserted seed data into database");
    }
}
