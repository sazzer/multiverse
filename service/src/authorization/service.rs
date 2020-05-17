use chrono::Duration;

mod generate_authorization;
mod token;

/// The Authorization Service to work with
#[derive(Clone)]
pub struct AuthorizationService {
    /// The duration of time that a generated authorization is valid for
    duration: Duration,
    /// The secret key used for the JWT generation
    secret: String,
}

impl AuthorizationService {
    /// Create a new instance of the Authorization Service
    ///
    /// # Parameters
    /// - `duration` - The duration of time that a generated authorization is valid for
    /// - `secret` - The secret key used for the JWT
    ///
    /// # Returns
    /// The Authorization Service ready to use
    pub(super) fn new(duration: Duration, secret: String) -> Self {
        AuthorizationService { duration, secret }
    }
}
