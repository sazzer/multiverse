use serde::{Deserialize, Serialize};

/// An Access Token that is used to access resources
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Token(String);

impl Token {
    /// Construct a new token wrapping the given string
    ///
    /// # Parameters
    /// - `value` - The value to wrap
    ///
    /// # Returns
    /// The token
    pub fn new<S>(value: S) -> Token
    where
        S: Into<String>,
    {
        Token(value.into())
    }
}

// Unfortunately this is needed so that the token can be deserialized
impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
