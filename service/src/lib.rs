mod infrastructure;
mod testing;

pub use infrastructure::server::testing::TestResponse;
pub use infrastructure::service::Service;
pub use testing::database::TestDatabase;

/// Representation of the configuration settings needed to build and run the application service
#[derive(Debug)]
pub struct Settings {
    /// The URL to connect to the database with
    pub database_url: String,
}
