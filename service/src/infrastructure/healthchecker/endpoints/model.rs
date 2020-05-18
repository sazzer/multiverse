use crate::infrastructure::healthchecker::{ComponentHealth, SystemHealth};
use rocket::{http::Status, response};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::collections::HashMap;

/// HTTP Representation of the health of a single component
///
/// # Examples
/// ## Healthy Component
/// ```json
/// {
///   "healthy": true
/// }
/// ## Unhealthy Component
/// ```json
/// {
///   "healthy": false,
///   "message": "Connection reset by peer"
/// }
#[derive(Serialize)]
pub struct ComponentHealthResponse {
    /// Whether this component is healthy or not
    pub healthy: bool,
    /// The error message if the component is unhealthy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// HTTP Representation of the health of the whole system
///
/// # Examples
/// ## Healthy System
/// ```json
/// {
///   "healthy": true,
///   "components": [
///     "db": {
///       "healthy": "true"    
///     }
///   ]
/// }
/// ## Unhealthy Component
/// ```json
/// {
///   "healthy": true,
///   "components": [
///     "db": {
///       "healthy": "false",
///       "message": "Connection reset by peer"
///     }
///   ]
/// }
#[derive(Serialize)]
pub struct SystemHealthResponse {
    /// Whether the system as a whole is healthy or not
    pub healthy: bool,
    /// The health of the individual components in the system
    pub components: HashMap<String, ComponentHealthResponse>,
}

impl From<SystemHealth> for SystemHealthResponse {
    fn from(health: SystemHealth) -> Self {
        SystemHealthResponse {
            healthy: health.is_healthy(),
            components: health
                .components
                .into_iter()
                .map(|(name, component)| {
                    let health = ComponentHealthResponse {
                        healthy: component.is_healthy(),
                        message: match component {
                            ComponentHealth::Healthy => None,
                            ComponentHealth::Unhealthy(msg) => Some(msg),
                        },
                    };
                    (name, health)
                })
                .collect(),
        }
    }
}

impl<'r> response::Responder<'r> for SystemHealthResponse {
    fn respond_to(self, req: &rocket::Request) -> response::Result<'r> {
        let status_code = if self.healthy {
            Status::Ok
        } else {
            Status::ServiceUnavailable
        };

        response::Response::build()
            .merge(Json(self).respond_to(req).unwrap())
            .status(status_code)
            .ok()
    }
}
