use super::{AuthorizationDetails, AuthorizationService, Token};
use crate::{
    http::problem::{Problem, ProblemType},
    users::UserID,
};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    State,
};

/// The means for endpoints to authorize that the actions being performed are allowed
#[derive(Debug)]
pub struct Authorizer {
    /// The actual authorization details to work with
    authorization: Option<AuthorizationDetails>,
}

/// The actual authorization process
pub struct Authorizing {
    /// The actual authorization details to work with
    authorization: Option<AuthorizationDetails>,
    /// The current authorization result
    result: Result<(), ()>,
}

impl Authorizer {
    /// Start the authorization process
    ///
    /// # Returns
    /// The DSL for authorizing this request
    pub fn authorize(&self) -> Authorizing {
        Authorizing {
            authorization: self.authorization.clone(),
            result: Ok(()),
        }
    }
}

impl Authorizing {
    /// Check if the current request is authorized but without any care of which user
    ///
    /// # Parameters
    /// - `self` - Consumes self
    ///
    /// # Returns
    /// A new DSL in the correct status after this test
    pub fn authorized(self) -> Self {
        Self {
            result: self
                .result
                .and_then(|_| self.authorization.clone().and(Some(())).ok_or(())),
            ..self
        }
    }

    /// Check if the current request is authorized for exactly the given User ID
    ///
    /// # Parameters
    /// - `self` - Consumes self
    /// - `user_id` - The User ID to look for
    ///
    /// # Returns
    /// A new DSL in the correct status after this test
    pub fn same_user(self, user_id: &UserID) -> Self {
        Self {
            result: self.result.and_then(|_| {
                self.authorization
                    .clone()
                    .filter(|d| &d.user_id == user_id)
                    .and(Some(()))
                    .ok_or(())
            }),
            ..self
        }
    }

    /// Finish authorization, returning the result
    ///
    /// # Parameters
    /// - `self` - Consumes self
    ///
    /// # Returns
    /// On success, returns the User ID that is authenticated, if there is one
    /// On authorization failure, returns a Problem response indicating this
    pub fn finish(self) -> Result<Option<UserID>, Problem> {
        self.result
            .map(|_| self.authorization.map(|auth| auth.user_id))
            .map_err(|_| Problem::new(AuthorizerProblemType {}, Status::Forbidden))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Authorizer {
    /// No error is ever returned
    type Error = ();

    /// Load the Authorizer from the HTTP Request
    ///
    /// This will read a Token from the `Authorization` header, ensure that it starts with the string "Bearer "
    /// and then pass the rest on to the `AuthorizationService` to decode.
    ///
    /// If any of this fails then we return a successful `Authorizer` with no token. This will then fail
    /// all the checks.
    /// If instead this succeeds then we return a successful `Authorizer` with the decoded token, so that we
    /// can perform checks against it.
    ///
    /// # Parameters
    /// - `request` - The HTTP Request to load the token from
    ///
    /// # Returns
    /// The authorizer
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let authorization_service = request.guard::<State<AuthorizationService>>().unwrap();

        let token = request
            .headers()
            .get_one("Authorization")
            .filter(|token| {
                tracing::trace!(token = ?token, "Read authorization header");
                token.starts_with("Bearer ")
            })
            .map(|token| token[7..].to_owned())
            .map(|token| Token::new(token))
            .and_then(|token| {
                let authorization = authorization_service.parse_token(&token);

                tracing::debug!(token = ?token, authorization = ?authorization, "Read authorization token");
                authorization.ok()
            });
        tracing::debug!(token = ?token, "Read authorization details");

        Outcome::Success(Self {
            authorization: token,
        })
    }
}

/// Problem Type for the RFC-7807 Problem to return if authorization actually fails
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
        "tag:multiverse,2020:problems/unauthorized"
    }
}
