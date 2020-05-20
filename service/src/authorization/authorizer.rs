use crate::http::problem::{Problem, ProblemType};
use crate::users::UserID;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

/// The means for endpoints to authorize that the actions being performed are allowed
#[derive(Debug)]
pub struct Authorizer {}

/// The actual authorization process
pub struct Authorizing {
    result: Result<(), ()>,
}

impl Authorizer {
    pub fn authorize(&self) -> Authorizing {
        // Err(Problem::new(AuthorizerProblemType {}, Status::Forbidden))
        Authorizing { result: Ok(()) }
    }
}

impl Authorizing {
    pub fn same_user(self, user_id: &UserID) -> Self {
        Self {
            result: self.result.and_then(|_| Ok(())),
        }
    }

    pub fn finish(self) -> Result<(), Problem> {
        self.result
            .map_err(|_| Problem::new(AuthorizerProblemType {}, Status::Forbidden))
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for Authorizer {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Outcome::Success(Self {})
    }
}

#[derive(Debug)]
struct AuthorizerProblemType {}

impl std::fmt::Display for AuthorizerProblemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An invalid access token was provided")
    }
}

impl ProblemType for AuthorizerProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        "tag:multiverse,2020:problems/invalid_access_token"
    }
}
