/// Representation of a page of results
#[derive(Debug)]
pub struct Page<T> {
    /// The actual entries on the page
    pub entries: Vec<T>,
    /// The offset of this page in the total resultset
    pub offset: u64,
    /// The count of records requested in a single page
    pub count: u64,
    /// The total number of records in the resultset
    pub total: u64,
}
