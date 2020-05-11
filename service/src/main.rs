use config::{Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// Representation of the application settings that will be loaded from the environment
#[derive(Deserialize)]
struct Settings {
    /// The port on which the HTTP server should listen on
    pub port: Option<u16>,
    /// The URL to connect to the database with
    pub database_url: String,
}

impl Default for Settings {
    /// Construct the settings from the environment
    ///
    /// # Returns
    /// The Settings object, loaded from the environment variables
    fn default() -> Self {
        let mut s = Config::new();
        s.merge(Environment::default())
            .expect("Failed to load environment properties");

        s.try_into().expect("Failed to build settings from config")
    }
}

impl Settings {
    /// Build the actual `Settings` object that the applicatin need to work with out of our settings
    /// loaded from the environment
    ///
    /// # Returns
    /// The settings needed to run the application
    pub fn build(&self) -> multiverse_lib::Settings {
        multiverse_lib::Settings {
            database_url: self.database_url.clone(),
        }
    }
}

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();

    if args.get(1).cloned() == Some("test".to_owned()) {
        tracing::info!("Testing application startup");
        return;
    }

    let settings = Settings::default();

    let service = multiverse_lib::Service::new(settings.build()).await;
    service.start(settings.port.unwrap_or(8000)).await;
}
