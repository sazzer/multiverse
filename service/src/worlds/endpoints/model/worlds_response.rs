use super::{to_json, WorldLink};
use crate::{
    http::link::{Link, LinkRel, Links},
    model::Page,
    users::{endpoints::model::UserLink, UserID, UserModel, UsersService},
    worlds::WorldModel,
};
use itertools::*;
use rocket::{
    http::hyper::header::{CacheControl, CacheDirective},
    response, Request, State,
};
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::collections::HashMap;

/// API Model representing a World
#[derive(Debug)]
pub struct WorldsResponse(pub Page<WorldModel>);

impl<'r> response::Responder<'r> for WorldsResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let users_service = req.guard::<State<UsersService>>().unwrap();

        let user_ids: Vec<&UserID> = self
            .0
            .entries
            .iter()
            .map(|world| &world.data.owner)
            .unique()
            .collect();
        let users: HashMap<UserID, UserModel> = users_service
            .find_users_by_id(&user_ids[..])
            .into_iter()
            .map(|user| (user.identity.id.clone(), user))
            .collect();

        let worlds: Vec<Value> = self.0.entries.iter().map(|world| to_json(&world)).collect();

        let response_body = json!({
          "entries": worlds,
          "pagination": {
            "offset": self.0.offset,
            "total": self.0.total
          }
        });

        let mut world_links: Vec<Link> = self
            .0
            .entries
            .iter()
            .map(|world| WorldLink::new(world.identity.id.clone()))
            .enumerate()
            .map(|(index, link)| {
                Link::new(link, LinkRel::ITEM).anchor(format!("#/entries/{}", index))
            })
            .collect();
        let user_links: Vec<Link> = self
            .0
            .entries
            .iter()
            .map(|world| &world.data.owner)
            .map(|owner| users.get(&owner))
            .enumerate()
            .filter(|(_, user)| user.is_some())
            .map(|(index, user)| (index, user.unwrap()))
            .map(|(index, user)| {
                (
                    index,
                    UserLink::new(user.identity.id.clone()),
                    user.data.display_name.clone(),
                )
            })
            .map(|(index, link, title)| {
                Link::new(link, LinkRel::AUTHOR)
                    .anchor(format!("#/entries/{}", index))
                    .title(title)
            })
            .collect();
        world_links.extend(user_links);

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
