use super::repository::UserRepository;

mod lookup_username;

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
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }
}
