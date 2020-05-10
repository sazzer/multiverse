use super::repository::UserRepository;

/// Service Layer for dealing with Users
#[derive(Clone)]
pub struct UsersService {
    pub(super) repository: UserRepository,
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
