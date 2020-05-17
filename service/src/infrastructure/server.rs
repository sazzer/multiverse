use rocket::{config::Environment, local::Client, Config, Rocket};
use std::sync::Arc;

/// A function that is able to contribute configuration to the Actix server when it is being constructed
pub type FnConfig = Arc<dyn Fn(Rocket) -> Rocket + Send + Sync>;

/// The actual HTTP Server that will be handling all of the web traffic
pub struct Server {
    configs: Vec<FnConfig>,
}

impl Server {
    /// Create a new web server that will be ready to process requests
    ///
    /// # Returns
    /// The web server, ready to work with
    pub fn new(configs: Vec<FnConfig>) -> Self {
        Server { configs }
    }

    /// Actually start the web server listening for incomiing HTTP connections
    ///
    /// # Parameters
    /// - `port` - The port to listen on
    pub fn start(&self, port: u16) {
        self.build(port).launch();
    }

    /// Build the Rocket instance to work with.
    ///
    /// This method exists so that the exact same instance can be used with both the live server and testing
    ///
    /// # Parameters
    /// - `port` - The port to listen on
    ///
    /// # Returns
    /// The rocket instance
    fn build(&self, port: u16) -> Rocket {
        let config =
            Config::build(Environment::active().expect("Invalid rocket environment specified"))
                .port(port)
                .finalize()
                .expect("Failed to create rocket config");

        let mut rocket = rocket::custom(config);

        for config in &self.configs {
            rocket = config(rocket);
        }

        rocket
    }
    /// Get a test client used to test the server
    ///
    /// # Returns
    /// The test client
    pub fn test_client(&self) -> Client {
        let rocket = self.build(0);

        Client::new(rocket).expect("valid rocket instance")
    }
}
