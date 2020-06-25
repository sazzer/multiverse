/// Representation of a page of results
#[derive(Debug)]
pub struct Page<T> {
    /// The actual entries on the page
    pub entries: Vec<T>,
    /// The offset of this page in the total resultset
    pub offset: u32,
    /// The total number of records in the resultset
    pub total: u32,
}
