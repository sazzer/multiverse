#![feature(proc_macro_hygiene, decl_macro)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::module_name_repetitions, clippy::wildcard_imports)
)]

mod authentication;
mod authorization;
mod http;
mod infrastructure;
mod model;
mod testing;
mod users;

pub use infrastructure::server::testing::TestResponse;
pub use infrastructure::service::Service;
pub use testing::database::TestDatabase;

/// Representation of the configuration settings needed to build and run the application service
#[derive(Debug)]
pub struct Settings {
    /// The URL to connect to the database with
    pub database_url: String,
}
