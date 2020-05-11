use super::World;
use std::sync::Arc;
use tokio_postgres::types::ToSql;

/// Trait that represents a type that can be seeded into the database
pub trait Seedable: std::fmt::Debug + Send + Sync {
    /// Generate the SQL needed to insert the seeded record into the database
    ///
    /// # Returns
    /// The SQL
    fn sql(&self) -> &str;

    /// Generate the binds needed to insert the seeded record into the database
    ///
    /// # Returns
    /// The binds
    fn binds(&self) -> Vec<&(dyn ToSql + Sync)>;
}

impl World {
    /// Insert some seed data into the database
    ///
    /// # Parameters
    /// - `data` - The data to seed into the database
    #[tracing::instrument(skip(self))]
    pub fn seed(&mut self, data: Arc<dyn Seedable>) {
        let mut rt = actix_rt::Runtime::new().unwrap();
        let service = self.service.clone();

        let data = data.clone();
        rt.block_on(async move { service.seed(data.clone()).await });
    }
}
