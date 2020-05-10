use serde::{Deserialize, Serialize};

/// Typesafe representation of the Username of some user
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Username(String);

impl Username {
    /// Construct a new Username from the given input value
    ///
    /// # Parameters
    /// - `username` - The username to wrap
    ///
    /// # Returns
    /// The wrapper Username
    pub fn new<S>(username: S) -> Self
    where
        S: Into<String>,
    {
        Self(username.into().trim().to_owned())
    }
}
