use multiverse_lib::{TestDatabase, TestResponse, Service, Settings};

/// Wrapper around the service that we are testing, allowing us to interact with it as needed.
///
/// This will start up the service including a database that it works with, and allow us to send
/// queries to the database and HTTP requests to the server, including the ability to seed data into
/// the database.
pub struct TestService {
    _database: TestDatabase,
    service: Service
}

impl TestService {
    /// Create a Test Service ready for us to test against
    ///
    /// # Returns
    /// A constructed service ready for us to test against
    pub async fn new() -> TestService {
        let database = TestDatabase::default();
        let settings = Settings {
            database_url: database.url.clone()
        };

        let service = multiverse_lib::Service::new(settings).await;

        TestService {
            _database: database, 
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
}
