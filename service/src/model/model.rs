use super::Identity;

/// The representation of some model resource
///
/// # Types
/// - `<ID>` - The type to use for the ID of the resource
/// - `<DATA>` - The type to use for the actual data
#[derive(Debug)]
pub struct Model<ID, DATA> {
    pub identity: Identity<ID>,
    pub data: DATA,
}
