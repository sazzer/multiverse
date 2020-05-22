use postgres::types::ToSql;

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
