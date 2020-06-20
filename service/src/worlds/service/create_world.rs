use crate::worlds::{repository::SaveWorldError, *};

/// Errors that can occur when creating a new user record
#[derive(Debug, thiserror::Error)]
pub enum CreateWorldError {
    /// An unknown error occurred
    #[error("An unknown error occurred")]
    UnknownError,

    #[error("The URL Slug was already present for this user")]
    DuplicateUrlSlug,

    #[error("The desired owner does not exist")]
    UnknownOwner,
}

impl WorldsService {
    /// Create a new world in the system
    ///
    /// # Parameters
    /// - `world` - The world to create
    pub fn create_world(&self, world: WorldData) -> Result<WorldModel, CreateWorldError> {
        tracing::debug!(world = ?world, "Creating world");

        let new_world = self.repository.create(world)?;
        tracing::debug!(world = ?new_world, "Created world");

        Ok(new_world)
    }
}

impl From<SaveWorldError> for CreateWorldError {
    fn from(e: SaveWorldError) -> Self {
        match e {
            SaveWorldError::DuplicateUrlSlug => CreateWorldError::DuplicateUrlSlug,
            SaveWorldError::UnknownOwner => CreateWorldError::UnknownOwner,
            _ => CreateWorldError::UnknownError,
        }
    }
}
