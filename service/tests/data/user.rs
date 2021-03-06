use crate::service::Seedable;
use argonautica::Hasher;
use chrono::{DateTime, Timelike, Utc};
use postgres::types::ToSql;
use uuid::Uuid;

/// A user that can be seeded into the database
#[derive(Debug, Clone)]
pub struct SeedUser {
    /// The ID of the user
    pub user_id: Uuid,
    /// The version of the user
    pub version: Uuid,
    /// When the user was created
    pub created: DateTime<Utc>,
    /// When the user was last updated
    pub updated: DateTime<Utc>,

    /// The username of the user
    pub username: String,
    /// The display name of the user
    pub display_name: String,
    /// The email address of the user
    pub email_address: String,
    /// The URL of the users avatar
    pub avatar_url: Option<String>,
    /// The users hashed password
    pub password: String,
}

pub fn hash_password<S>(input: S) -> String
where
    S: Into<String>,
{
    Hasher::default()
        .with_password(&input.into())
        .opt_out_of_secret_key(true)
        .hash()
        .unwrap()
}

impl Default for SeedUser {
    fn default() -> Self {
        let now = Utc::now().with_nanosecond(0).unwrap();

        Self {
            user_id: Uuid::new_v4(),
            version: Uuid::new_v4(),
            created: now,
            updated: now,

            username: format!("username_{}", Uuid::new_v4()),
            display_name: format!("Display Name {}", Uuid::new_v4()),
            email_address: format!("{}@example.com", Uuid::new_v4()),
            avatar_url: None,
            password: hash_password("password"),
        }
    }
}

impl Seedable for SeedUser {
    /// Generate the SQL needed to insert the seeded record into the database
    ///
    /// # Returns
    /// The SQL
    fn sql(&self) -> &str {
        "INSERT INTO users(user_id, version, created, updated, username, display_name, email_address, avatar_url, password)
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    }

    /// Generate the binds needed to insert the seeded record into the database
    ///
    /// # Returns
    /// The binds
    fn binds(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id,
            &self.version,
            &self.created,
            &self.updated,
            &self.username,
            &self.display_name,
            &self.email_address,
            &self.avatar_url,
            &self.password,
        ]
    }
}
