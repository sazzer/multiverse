use super::check_health::CheckHealth;
use super::model::{ComponentHealth, SystemHealth};
use std::{collections::HashMap, sync::Arc};

/// System to check the health of the application
pub struct Healthchecker {
    /// The set of components whos health is to be checked
    components: HashMap<String, Arc<dyn CheckHealth>>,
}

impl Healthchecker {
    /// Construct a new healthchecker to check the health of the system
    ///
    /// # Parameters
    /// - `components` - The components to check as part of the system
    ///
    /// # Returns
    /// The newly constructed health checker to work with
    pub fn new(components: HashMap<String, Arc<dyn CheckHealth>>) -> Self {
        Self { components }
    }

    /// Check the health of the entire system.
    ///
    /// This calls each of the individual healthchecks that are registered and uses the results from those
    /// to compile the overall health of the system
    ///
    /// # Returns
    /// The health of the system as a whole
    pub async fn check_health(&self) -> SystemHealth {
        let mut result = HashMap::new();

        for (name, component) in &self.components {
            let component_health = component.check_health().await;
            tracing::debug!(component = ?name, health = ?component_health, "Component health");
            result.insert(
                name.clone(),
                match component_health {
                    Ok(_) => ComponentHealth::Healthy,
                    Err(e) => ComponentHealth::Unhealthy(format!("{}", e)),
                },
            );
        }

        SystemHealth::new(result)
    }
}

/// Builder to allow easier construction of a `Healthchecker` by allowing components to be added individually
#[derive(Default, Clone)]
pub struct HealthcheckerBuilder {
    /// The set of components whos health is to be checked
    components: HashMap<String, Arc<dyn CheckHealth>>,
}

impl HealthcheckerBuilder {
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
    pub fn with_component<S>(&mut self, name: S, component: Arc<dyn CheckHealth>) -> &mut Self
    where
        S: Into<String>,
    {
        self.components.insert(name.into(), component);

        self
    }

    /// Actually build the health checker
    ///
    /// # Returns
    /// A Healthchecker instance that represents the same set of components
    pub fn build(self) -> Healthchecker {
        Healthchecker::new(self.components)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_no_components() {
        let builder = HealthcheckerBuilder::default();
        // builder.with_component("passing", Arc::new(Ok(())));
        let sut = builder.build();

        let result = sut.check_health().await;
        assert_eq!(true, result.is_healthy());
        assert_eq!(true, result.components.is_empty());
    }

    #[actix_rt::test]
    async fn test_passing_components() {
        let mut builder = HealthcheckerBuilder::default();
        builder.with_component("passing", Arc::new(Ok(())));
        let sut = builder.build();

        let result = sut.check_health().await;
        assert_eq!(true, result.is_healthy());
        assert_eq!(1, result.components.len());
        assert_eq!(
            Some(&ComponentHealth::Healthy),
            result.components.get("passing")
        );
    }

    #[actix_rt::test]
    async fn test_failing_components() {
        let mut builder = HealthcheckerBuilder::default();
        builder.with_component("failing", Arc::new(Err("Oops")));
        let sut = builder.build();

        let result = sut.check_health().await;
        assert_eq!(false, result.is_healthy());
        assert_eq!(1, result.components.len());
        assert_eq!(
            Some(&ComponentHealth::Unhealthy("Oops".to_owned())),
            result.components.get("failing")
        );
    }

    #[actix_rt::test]
    async fn test_mixed_components() {
        let mut builder = HealthcheckerBuilder::default();
        builder.with_component("passing", Arc::new(Ok(())));
        builder.with_component("failing", Arc::new(Err("Oops")));
        let sut = builder.build();

        let result = sut.check_health().await;
        assert_eq!(false, result.is_healthy());
        assert_eq!(2, result.components.len());
        assert_eq!(
            Some(&ComponentHealth::Healthy),
            result.components.get("passing")
        );
        assert_eq!(
            Some(&ComponentHealth::Unhealthy("Oops".to_owned())),
            result.components.get("failing")
        );
    }
}
