mod create_user;
mod find;
mod update;

use super::repository::UserRepository;
pub use create_user::CreateUserError;

/// Service Layer for dealing with Users
#[derive(Clone)]
pub struct UsersService {
    repository: UserRepository,
}

impl UsersService {
    /// Create a new instance of the Users Service
    ///
    /// # Returns
    /// The Users Service ready to use
    pub(super) fn new(repository: UserRepository) -> Self {
        Self { repository }
    }
}
