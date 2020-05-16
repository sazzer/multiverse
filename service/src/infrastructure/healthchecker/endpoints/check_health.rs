use super::model::SystemHealthModel;
use crate::infrastructure::healthchecker::Healthchecker;
use rocket::get;

/// Actix handler to check the health of the system
///
/// # Parameters
/// - `healthchecker` - The service used to check the health of the system
///
/// # Returns
/// The API model representing the health of the system
#[tracing::instrument(name = "GET /health", skip(healthchecker))]
#[get("/health")]
pub fn check_health(healthchecker: rocket::State<Healthchecker>) -> SystemHealthModel {
    let health = healthchecker.check_health();

    health.into()
}
