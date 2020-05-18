use super::model::SystemHealthResponse;
use crate::infrastructure::healthchecker::Healthchecker;
use rocket::{get, State};

/// Actix handler to check the health of the system
///
/// # Parameters
/// - `healthchecker` - The service used to check the health of the system
///
/// # Returns
/// The API model representing the health of the system
#[tracing::instrument(name = "GET /health", skip(healthchecker))]
#[get("/health")]
pub fn check_health(healthchecker: State<Healthchecker>) -> SystemHealthResponse {
    let health = healthchecker.check_health();

    health.into()
}
