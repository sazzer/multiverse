use actix_web::web;
use std::sync::Arc;

/// Application Configuration for the Users module
#[derive(Default)]
pub struct UsersConfig {}

impl UsersConfig {
    /// Generate the configuration callback needed for the HTTP Server to actually add the Users endpoints
    /// to the server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the Users endpoints
    pub fn configure(&self) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
        Arc::new(move |config| {
            config.service(super::endpoints::lookup_username);
        })
    }
}
