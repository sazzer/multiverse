use crate::{
    authorization::Authorizer,
    http::problem::{GenericValidation, Problem, ProblemType, ValidationProblem},
    worlds::{
        endpoints::model::WorldResponse, CreateWorldError, UrlSlug, UrlSlugParseError, WorldData,
        WorldsService,
    },
};
use rocket::{http::Status, post, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use str_slug::slug;

/// Handler to create a new World
///
/// # Parameters
/// - `worlds_service` - The worlds service to use
/// - `body` - The details of the world to create
/// - `authorizer` - The authorizer to prove we're allowed to create a world
///
/// # Returns
/// The newly created world details, or a Problem if the creation failed
#[tracing::instrument(name = "POST /worlds", skip(worlds_service))]
#[post("/worlds", data = "<body>")]
pub fn create_world(
    worlds_service: State<WorldsService>,
    body: Json<CreateWorldRequest>,
    authorizer: Authorizer,
) -> Result<WorldResponse, Problem> {
    let owner = authorizer.authorize().authorized().finish()?;

    let name = body.name();
    let description = body.description();
    let url_slug = body.url_slug();

    tracing::debug!(
        name = ?name,
        description = ?description,
        url_slug = ?url_slug,
        "Creating new world"
    );

    match (&name, &url_slug, &owner) {
        (Some(name), Ok(url_slug), Some(owner)) => {
            // Try to create the world
            let new_world = worlds_service.create_world(WorldData {
                name: name.clone(),
                description: description.unwrap_or("".to_owned()),
                url_slug: url_slug.clone(),
                owner: owner.clone(),
            })?;
            Ok(WorldResponse(new_world))
        }
        (_, _, None) => {
            tracing::error!("No authenticated user");
            unreachable!()
        }
        _ => {
            // Return a validation problem
            tracing::warn!("Validation error creating world");

            let mut problem = ValidationProblem::new();

            if name == None {
                problem.with_field_error("name", GenericValidation::Missing);
            }

            if let Err(err) = url_slug.map_err(|e| match e {
                UrlSlugParseError::Blank => GenericValidation::Missing,
            }) {
                problem.with_field_error("url_slug", err);
            }

            Err(problem.build())
        }
    }
}

/// Incoming details representing a request to create a new world
#[derive(Debug, Deserialize)]
pub struct CreateWorldRequest {
    /// The name of the world
    name: Option<String>,
    /// The description of the world. May be omitted
    description: Option<String>,
    /// The URL Slug of the world. If omitted then this is generated from the name
    url_slug: Option<String>,
}

impl CreateWorldRequest {
    /// Get the name to use for the world
    fn name(&self) -> Option<String> {
        self.name.clone().filter(|v| !v.trim().is_empty())
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
            .unwrap_or_else(|| slug(self.name.clone().unwrap_or("".to_owned())))
            .parse()
    }
}

/// Problem Types that can happen when registering a user
#[derive(Debug, thiserror::Error)]
pub enum CreateWorldProblemType {
    /// An unknown error occurred
    #[error("An unknown error occurred")]
    UnknownError,

    #[error("The URL Slug was already present for this user")]
    DuplicateUrlSlug,
}

impl ProblemType for CreateWorldProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        match self {
            CreateWorldProblemType::UnknownError => {
                "tag:multiverse,2020:worlds/problems/unknown_error"
            }
            CreateWorldProblemType::DuplicateUrlSlug => {
                "tag:multiverse,2020:worlds/problems/duplicate_url_slug"
            }
        }
    }
}

impl From<CreateWorldError> for Problem {
    fn from(e: CreateWorldError) -> Self {
        match e {
            CreateWorldError::DuplicateUrlSlug => Problem::new(
                CreateWorldProblemType::DuplicateUrlSlug,
                Status::UnprocessableEntity,
            ),
            _ => {
                tracing::warn!(error = ?e, "An unexpected error occurred");
                Problem::new(
                    CreateWorldProblemType::UnknownError,
                    Status::InternalServerError,
                )
            }
        }
    }
}
