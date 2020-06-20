use super::WorldLink;
use crate::{
    http::link::{Link, LinkRel, Links},
    users::endpoints::model::UserLink,
    worlds::WorldModel,
};
use rocket::{
    http::hyper::header::{CacheControl, CacheDirective, ETag, EntityTag},
    response, Request,
};
use rocket_contrib::json::Json;
use serde_json::json;
/// API Model representing a World
#[derive(Debug)]
pub struct WorldResponse(pub WorldModel);

impl<'r> response::Responder<'r> for WorldResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let etag = self.0.identity.version.to_string();
        let response_model = json!({
          "name": self.0.data.name,
          "description": self.0.data.description,
          "url_slug": self.0.data.url_slug
        });

        response::Response::build()
            .merge(Json(response_model).respond_to(req).unwrap())
            .header(CacheControl(vec![
                CacheDirective::Private,
                CacheDirective::MaxAge(3600),
            ]))
            .header(ETag(EntityTag::new(false, etag)))
            .header(Links(vec![
                Link::new(WorldLink::new(self.0.identity.id), LinkRel::SELF),
                Link::new(UserLink::new(self.0.data.owner), LinkRel::AUTHOR),
            ]))
            .ok()
    }
}
