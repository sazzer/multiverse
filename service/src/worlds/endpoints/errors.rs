use crate::http::problem::ProblemType;

/// Problem Types that can happen when working with worlds
#[derive(Debug, thiserror::Error)]
pub enum WorldProblemType {
    /// The world ID that was looked up was not found
    #[error("The requested world ID was unknown")]
    UnknownWorldID,
}

impl ProblemType for WorldProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        match self {
            WorldProblemType::UnknownWorldID => {
                "tag:multiverse,2020:worlds/problems/unknown_world_id"
            }
        }
    }
}
