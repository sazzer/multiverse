use async_trait::async_trait;
use std::error::Error;

/// Trait that any component capable of reporting on it's health may implement in order to do so
#[async_trait]
pub trait CheckHealth: Send + Sync {
    /// Actually check the health of the component, and return any errors that have occurred with it.
    ///
    /// # Returns
    /// In the event that the component is healthy, a void value is returned.
    ///
    /// # Errors
    /// In the event that the component is unhealthy, an error indicating the actual error condition with
    /// the component is returned. This is any error type as appropriate to the component.
    async fn check_health(&self) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
#[async_trait]
impl CheckHealth for Result<(), &str> {
    async fn check_health(&self) -> Result<(), Box<dyn Error>> {
        self.clone()
            .map_err(|e| Box::new(simple_error::SimpleError::new(e)) as Box<dyn Error>)
    }
}
