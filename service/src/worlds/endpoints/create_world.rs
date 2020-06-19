use crate::{
    http::problem::Problem,
    worlds::{UrlSlug, UrlSlugParseError, WorldsService},
};
use rocket::{post, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use str_slug::slug;

/// Handler to create a new World
///
/// # Parameters
/// - `worlds_service` - The worlds service to use
///
/// # Returns
/// The newly created world details, or a Problem if the creation failed
#[tracing::instrument(name = "GET /usernames/{id}", skip(worlds_service))]
#[post("/worlds", data = "<body>")]
pub fn create_world(
    worlds_service: State<WorldsService>,
    body: Json<CreateWorldRequest>,
) -> Result<String, Problem> {
    let name = body.name();
    let description = body.description();
    let url_slug = body.url_slug();

    tracing::debug!(
        name = ?name,
        description = ?description,
        url_slug = ?url_slug,
        "Creating new world"
    );

    todo!()
}

/// Incoming details representing a request to create a new world
#[derive(Debug, Deserialize)]
pub struct CreateWorldRequest {
    /// The name of the world
    name: String,
    /// The description of the world. May be omitted
    description: Option<String>,
    /// The URL Slug of the world. If omitted then this is generated from the name
    url_slug: Option<String>,
}

impl CreateWorldRequest {
    /// Get the name to use for the world
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Get the description to use for the world
    fn description(&self) -> Option<String> {
        self.description.clone().filter(|v| !v.trim().is_empty())
    }

    /// Get the URL Slug to use for the world
    /// If one wasn't provided then it will be generated from the name
    fn url_slug(&self) -> Result<UrlSlug, UrlSlugParseError> {
        self.url_slug
            .clone()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| slug(&self.name))
            .parse()
    }
}
