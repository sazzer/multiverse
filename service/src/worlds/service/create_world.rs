use crate::worlds::*;

impl WorldsService {
    /// Create a new world in the system
    ///
    /// # Parameters
    /// - `world` - The world to create
    pub fn create_world(&self, world: WorldData) {
        tracing::debug!(world = ?world, "Creating world");
        todo!();
    }
}
