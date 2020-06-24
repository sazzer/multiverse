/// Pagiation details for searching for entities
#[derive(Debug, Copy, Clone)]
pub struct Pagination {
    /// The desired offset for the request
    pub offset: u32,
    /// The desired count for the request
    pub count: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            count: 10,
        }
    }
}
