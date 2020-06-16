use super::{UrlSlug, WorldID};
use crate::{model::Model, users::UserID};

/// Data to represent a world
#[derive(Debug, Clone)]
pub struct WorldData {
    /// The name of the world
    pub name: String,

    /// The ID of the user that owns the world
    pub owner: UserID,

    /// A description of the world
    pub description: String,

    /// The URL Slug of the world
    pub url_slug: UrlSlug,
}

/// Model representation of a World
pub type WorldModel = Model<WorldID, WorldData>;
