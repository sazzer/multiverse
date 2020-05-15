use serde::{Deserialize, Serialize};

/// An Access Token that is used to access resources
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Token(String);
