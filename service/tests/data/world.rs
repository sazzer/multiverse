use crate::service::Seedable;
use chrono::{DateTime, Timelike, Utc};
use postgres::types::ToSql;
use uuid::Uuid;

/// A world that can be seeded into the database
#[derive(Debug, Clone)]
pub struct SeedWorld {
    /// The ID of the world
    pub world_id: Uuid,
    /// The version of the world
    pub version: Uuid,
    /// When the world was created
    pub created: DateTime<Utc>,
    /// When the world was last updated
    pub updated: DateTime<Utc>,

    /// The name of the world
    pub name: String,
    /// The description of the world
    pub description: String,
    /// The URL Slug of the world
    pub url_slug: String,
    /// The User ID of the owner of the world
    pub owner: Uuid,
}

impl Default for SeedWorld {
    fn default() -> Self {
        let now = Utc::now().with_nanosecond(0).unwrap();

        Self {
            world_id: Uuid::new_v4(),
            version: Uuid::new_v4(),
            created: now,
            updated: now,

            name: format!("name_{}", Uuid::new_v4()),
            description: format!("Description {}", Uuid::new_v4()),
            url_slug: format!("url_slug_{}", Uuid::new_v4()),
            owner: Uuid::new_v4(),
        }
    }
}

impl Seedable for SeedWorld {
    /// Generate the SQL needed to insert the seeded record into the database
    ///
    /// # Returns
    /// The SQL
    fn sql(&self) -> &str {
        "INSERT INTO worlds(world_id, version, created, updated, name, description, url_slug, owner_id)
        VALUES($1, $2, $3, $4, $5, $6, $7, $8)"
    }

    /// Generate the binds needed to insert the seeded record into the database
    ///
    /// # Returns
    /// The binds
    fn binds(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.world_id,
            &self.version,
            &self.created,
            &self.updated,
            &self.name,
            &self.description,
            &self.url_slug,
            &self.owner,
        ]
    }
}
