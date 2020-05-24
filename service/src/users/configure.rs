use super::{repository::UserRepository, UsersService};
use crate::infrastructure::database::Database;
use std::sync::Arc;

/// Application Configuration for the Users module
pub struct UsersConfig {
    pub users_service: UsersService,
}

impl UsersConfig {
    /// Create the Configuration for the Users Module
    ///
    /// # Returns
    /// The Users Config object
    pub fn new(database: Database) -> Self {
        let repository = UserRepository::new(database);
        Self {
            users_service: UsersService::new(repository),
        }
    }

    /// Generate the configuration callback needed for the HTTP Server to actually add the Users endpoints
    /// to the server
    ///
    /// # Returns
    /// The callback to provide to the HTTP Server to configure up the Users endpoints
    pub fn configure(&self) -> Arc<dyn Fn(rocket::Rocket) -> rocket::Rocket + Send + Sync> {
        let users_service = self.users_service.clone();
        Arc::new(move |config| {
            config.manage(users_service.clone()).mount(
                "/",
                rocket::routes![
                    super::endpoints::lookup_user,
                    super::endpoints::patch_user,
                    super::endpoints::lookup_username
                ],
            )
        })
    }
}
