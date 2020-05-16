use super::model::SystemHealthModel;
use crate::infrastructure::healthchecker::Healthchecker;
use actix_web::{get, web};

/// Actix handler to check the health of the system
///
/// # Parameters
/// - `healthchecker` - The service used to check the health of the system
///
/// # Returns
/// The API model representing the health of the system
#[tracing::instrument(name = "GET /health", skip(healthchecker))]
#[get("/health")]
pub async fn check_health(healthchecker: web::Data<Healthchecker>) -> SystemHealthModel {
    let health = healthchecker.check_health();

    health.into()
}
