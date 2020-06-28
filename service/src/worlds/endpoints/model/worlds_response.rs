use super::{to_json, WorldLink};
use crate::{
    http::link::{Link, LinkRel, Links},
    model::Page,
    worlds::WorldModel,
};
use rocket::{
    http::hyper::header::{CacheControl, CacheDirective},
    response, Request,
};
use rocket_contrib::json::Json;
use serde_json::{json, Value};

/// API Model representing a World
#[derive(Debug)]
pub struct WorldsResponse(pub Page<WorldModel>);

impl<'r> response::Responder<'r> for WorldsResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let worlds: Vec<Value> = self.0.entries.iter().map(|world| to_json(&world)).collect();

        let response_body = json!({
          "entries": worlds,
          "pagination": {
            "offset": self.0.offset,
            "total": self.0.total
          }
        });

        let world_links: Vec<Link> = self
            .0
            .entries
            .iter()
            .map(|world| WorldLink::new(world.identity.id.clone()))
            .enumerate()
            .map(|(index, link)| {
                Link::new(link, LinkRel::ITEM).anchor(format!("#/entries/{}", index))
            })
            .collect();

        response::Response::build()
            .merge(Json(response_body).respond_to(req).unwrap())
            .header(CacheControl(vec![
                CacheDirective::Private,
                CacheDirective::MaxAge(3600),
            ]))
            .header(Links(world_links))
            .ok()
    }
}
