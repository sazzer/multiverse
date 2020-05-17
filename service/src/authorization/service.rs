use chrono::Duration;

mod generate_authorization;

/// The Authorization Service to work with
#[derive(Clone)]
pub struct AuthorizationService {
    /// The duration of time that a generated authorization is valid for
    duration: Duration,
}

impl AuthorizationService {
    /// Create a new instance of the Authorization Service
    ///
    /// # Parameters
    /// - `duration` - The duration of time that a generated authorization is valid for
    ///
    /// # Returns
    /// The Authorization Service ready to use
    pub(super) fn new(duration: Duration) -> Self {
        AuthorizationService { duration }
    }
}
