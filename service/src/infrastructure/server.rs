pub mod testing;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::{ops::Deref, sync::Arc};

/// A function that is able to contribute configuration to the Actix server when it is being constructed
pub type FnConfig = Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync>;

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
    pub async fn start(&self, port: u16) {
        let configs = self.configs.clone();
        let builder = move || {
            let configs = configs.clone();
            let mut app = App::new()
                .wrap(Logger::default())
                .wrap(Cors::new().finish());
            for config in &configs {
                app = app.configure(config.deref());
            }
            app
        };

        let listen_address = format!("0.0.0.0:{}", port);

        tracing::info!(address = ?listen_address, "Starting web server");

        HttpServer::new(builder)
            .bind(listen_address)
            .expect("Failed to bind to address")
            .run()
            .await
            .expect("Failed to start server");
    }
}
