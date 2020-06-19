use crate::worlds::WorldID;

/// Representation of a link to a world
pub struct WorldLink(WorldID);

impl WorldLink {
    pub fn new(world_id: WorldID) -> Self {
        Self(world_id)
    }
}

impl From<WorldLink> for String {
    fn from(world_link: WorldLink) -> String {
        format!("/worlds/{}", world_link.0)
    }
}
