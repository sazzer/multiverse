use std::collections::HashMap;

/// Enumeration of the health statuses of an individual component in the system
#[derive(Debug, PartialEq)]
pub enum ComponentHealth {
    /// The component is healthy
    Healthy,

    /// The component is unhealthy. This enumerated value includes an error message indicating
    /// exactly why the component is unhealthy
    Unhealthy(String),
}

impl ComponentHealth {
    /// Report on whether this component is healthy or not.
    ///
    /// # Returns
    /// `true` if this component is healthy. `false` if not.
    pub fn is_healthy(&self) -> bool {
        match self {
            ComponentHealth::Healthy => true,
            ComponentHealth::Unhealthy(_) => false,
        }
    }
}

/// Representation of the health of the entire system
pub struct SystemHealth {
    /// The health status of all the individual components in the system
    pub components: HashMap<String, ComponentHealth>,
}

impl SystemHealth {
    /// Construct a new `SystemHealth` instance, representing the component health of the provided set
    /// of components
    ///
    /// # Parameters
    /// - `components` - The individual components in the system
    ///
    /// # Returns
    /// The `SystemHealth` instance wrapping these components
    pub fn new(components: HashMap<String, ComponentHealth>) -> Self {
        Self { components }
    }

    /// Report on whether the system as a whole is healthy or not.
    ///
    /// The system is healthy if every individual component is healthy, and is not healthy if any
    /// of the individual components is unhealthy
    ///
    /// # Returns
    /// `true` if every individual component is healthy, or `false` if at least one is unhealthy
    pub fn is_healthy(&self) -> bool {
        self.components.iter().all(|(_, v)| v.is_healthy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy_component() {
        let sut = ComponentHealth::Healthy;
        assert_eq!(true, sut.is_healthy());
    }

    #[test]
    fn test_unhealthy_component() {
        let sut = ComponentHealth::Unhealthy("Oops".to_owned());
        assert_eq!(false, sut.is_healthy());
    }

    #[test]
    fn test_empty_system() {
        let sut = SystemHealth::new(HashMap::new());
        assert_eq!(true, sut.is_healthy());
    }

    #[test]
    fn test_healthy_system() {
        let mut components = HashMap::new();
        components.insert("healthy".to_owned(), ComponentHealth::Healthy);
        let sut = SystemHealth::new(components);
        assert_eq!(true, sut.is_healthy());
    }

    #[test]
    fn test_unhealthy_system() {
        let mut components = HashMap::new();
        components.insert(
            "unhealthy".to_owned(),
            ComponentHealth::Unhealthy("Oops".to_owned()),
        );
        let sut = SystemHealth::new(components);
        assert_eq!(false, sut.is_healthy());
    }

    #[test]
    fn test_mixed_system() {
        let mut components = HashMap::new();
        components.insert("healthy".to_owned(), ComponentHealth::Healthy);
        components.insert(
            "unhealthy".to_owned(),
            ComponentHealth::Unhealthy("Oops".to_owned()),
        );
        let sut = SystemHealth::new(components);
        assert_eq!(false, sut.is_healthy());
    }
}
