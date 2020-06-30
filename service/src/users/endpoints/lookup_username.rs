use super::{errors::UserProblemType, model::UserLink};
use crate::{
    http::{
        link::{Link, LinkRel, Links},
        problem::Problem,
    },
    users::{Username, UsersService},
};
use rocket::{
    get,
    http::{
        hyper::header::{CacheControl, CacheDirective},
        Status,
    },
    Response, State,
};

/// Actix handler to see if a username is already registered or not
///
/// # Parameters
/// - `user_service` - The user service to use to look up the username
/// - `path` - The details of the parameters from the URL
///
/// # Returns
/// If the username is known then an empty response.
/// If the username is not registered then an RFC-7807 problem response indicating this.
#[tracing::instrument(name = "GET /usernames/{username}", skip(users_service))]
#[get("/usernames/<username>")]
pub fn lookup_username(
    users_service: State<UsersService>,
    username: Username,
) -> Result<Response, Problem> {
    users_service
        .find_user_by_username(&username)
        .ok_or_else(|| Problem::new(UserProblemType::UnknownUsername, Status::NotFound))
        .map(|user| {
            Response::build()
                .status(Status::NoContent)
                .header(CacheControl(vec![
                    CacheDirective::Private,
                    CacheDirective::MaxAge(3600),
                ]))
                .header(Links(vec![Link::new(
                    UserLink::new(user.identity.id),
                    LinkRel::RELATED,
                )
                .title(user.data.display_name)]))
                .finalize()
        })
}
