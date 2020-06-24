/// Fields by which the results of sorting for worlds can be sorted
#[derive(Debug, strum_macros::EnumVariantNames, strum_macros::EnumString)]
pub enum WorldSortField {
    /// Sort by the name of the world
    Name,
    /// Sort by the name of the owner
    Owner,
    /// Sort by when the world was created
    Created,
    /// Sort by when the world was last updated
    Updated,
}
