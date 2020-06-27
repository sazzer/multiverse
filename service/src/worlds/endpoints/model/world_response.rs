use super::WorldLink;
use crate::{
    http::link::{Link, LinkRel, Links},
    users::{endpoints::model::UserLink, UsersService},
    worlds::WorldModel,
};
use rocket::{
    http::hyper::header::{CacheControl, CacheDirective, ETag, EntityTag},
    response, Request, State,
};
use rocket_contrib::json::Json;
use serde_json::{json, Value};

/// API Model representing a World
#[derive(Debug)]
pub struct WorldResponse(pub WorldModel);

pub(super) fn to_json(world: &WorldModel) -> Value {
    json!({
      "name": world.data.name,
      "description": world.data.description,
      "url_slug": world.data.url_slug
    })
}
impl<'r> response::Responder<'r> for WorldResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let users_service = req.guard::<State<UsersService>>().unwrap();
        let user = users_service.find_user_by_id(&self.0.data.owner);
        let user_name = user.map(|u| u.data.display_name).unwrap_or_default();

        let etag = self.0.identity.version.to_string();

        response::Response::build()
            .merge(Json(to_json(&self.0)).respond_to(req).unwrap())
            .header(CacheControl(vec![
                CacheDirective::Private,
                CacheDirective::MaxAge(3600),
            ]))
            .header(ETag(EntityTag::new(false, etag)))
            .header(Links(vec![
                Link::new(WorldLink::new(self.0.identity.id), LinkRel::SELF),
                Link::new(UserLink::new(self.0.data.owner), LinkRel::AUTHOR).title(user_name),
            ]))
            .ok()
    }
}
