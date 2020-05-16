pub(crate) mod testing;

use std::sync::Arc;

/// A function that is able to contribute configuration to the Actix server when it is being constructed
pub type FnConfig = Arc<dyn Fn(rocket::Rocket) -> rocket::Rocket + Send + Sync>;

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
        let config = rocket::Config::build(
            rocket::config::Environment::active().expect("Invalid rocket environment specified"),
        )
        .port(port)
        .finalize()
        .expect("Failed to create rocket config");

        let mut rocket = rocket::custom(config);

        for config in &self.configs {
            rocket = config(rocket);
        }

        rocket.launch();
    }
}
