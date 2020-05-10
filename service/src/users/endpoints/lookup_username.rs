use crate::http::problem::{Problem, ProblemType};
use crate::users::{Username, UsersService};
use actix_web::{get, http::StatusCode, web, Either, HttpResponse, Responder};

#[derive(Debug, thiserror::Error)]
pub enum LookupUsernameProblemType {
    #[error("The requested username was unknown")]
    UnknownUsername,
}

impl ProblemType for LookupUsernameProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        match self {
            LookupUsernameProblemType::UnknownUsername => {
                "tag:multiverse,2020:users/problems/unknown_username"
            }
        }
    }
}
/// Actix handler to see if a username is already registered or not
///
/// # Parameters
/// - `path` - The details of the parameters from the URL
///
/// # Returns
/// If the username is known then an empty response.
/// If the username is not registered then an RFC-7807 problem response indicating this.
#[tracing::instrument(name = "GET /usernames/{username}", skip(users_service))]
#[get("/usernames/{username}")]
pub async fn lookup_username(
    users_service: web::Data<UsersService>,
    path: web::Path<(Username,)>,
) -> Either<impl Responder, Problem<LookupUsernameProblemType>> {
    tracing::info!("Hello");

    let found = users_service.lookup_username(&path.0);

    if found {
        Either::A(HttpResponse::NoContent())
    } else {
        Either::B(Problem::new(
            LookupUsernameProblemType::UnknownUsername,
            StatusCode::NOT_FOUND,
        ))
    }
}
