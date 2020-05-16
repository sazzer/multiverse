use super::{CheckHealth, HealthcheckerBuilder};
use std::sync::Arc;

/// Application Configuration for the Healthchecks, allowing us to use them with other parts of the system
#[derive(Default)]
pub struct HealthcheckConfig {
    /// The builder used to actually build the healthchecker
    builder: HealthcheckerBuilder,
}

impl HealthcheckConfig {
    /// Add a new component to the health checker that we are building
    ///
    /// # Parameters
    /// - `name` - The name of the component to add
    /// - `component` - The actual component to add
    ///
    /// # Returns
    /// The builder, so that this call can be chained
    ///
    /// #Type Parameters
    /// - `<S>` - Anything that can convert into a `String`, to allow for easier calling
    pub fn with_component<S>(mut self, name: S, component: Arc<dyn CheckHealth>) -> Self
    where
        S: Into<String>,
    {
        self.builder.with_component(name, component);
        self
    }

    /// Generate the configuration callback needed for the HTTP Server to actually add the healthchecks
    /// on to the web server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the healthchecks
    pub fn configure(&self) -> Arc<dyn Fn(rocket::Rocket) -> rocket::Rocket + Send + Sync> {
        let builder = self.builder.clone();
        Arc::new(move |config| {
            let healthchecker = builder.clone().build();

            config.manage(healthchecker)
            // config.service(super::endpoints::check_health);
        })
    }
}
