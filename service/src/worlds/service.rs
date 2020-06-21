mod create_world;
mod find;
use super::repository::WorldRepository;
pub use create_world::CreateWorldError;

/// Service Layer for dealing with Worlds
#[derive(Clone)]
pub struct WorldsService {
    repository: WorldRepository,
}

impl WorldsService {
    /// Create a new instance of the Worlds Service
    ///
    /// # Returns
    /// The Worlds Service ready to use
    pub(super) fn new(repository: WorldRepository) -> Self {
        Self { repository }
    }
}
