use rocket::request::{FromQuery, Query};
use std::str::FromStr;

/// Request details to describe the pagination of a request
#[derive(Debug)]
pub struct PaginationRequest {
    /// The desired offset for the request
    pub offset: u32,
    /// The desired count for the request
    pub count: u32,
}

impl Default for PaginationRequest {
    fn default() -> Self {
        Self {
            offset: 0,
            count: 10,
        }
    }
}

impl<'q> FromQuery<'q> for PaginationRequest {
    /// No error is ever returned
    type Error = ();

    /// Load the pagination details from the request and return an appropriate struct.
    ///
    /// This always uses standard querystring parameters for the values, namely:
    /// - `offset`
    /// - `count`
    ///
    /// If these are not present then defaults will be used instead, as defined in the implementation of
    /// the `Default` trait for the `PaginationRequest` trait
    ///
    /// # Parameters
    /// - `query` - The query parameters to get the parameters from
    ///
    /// # Returns
    /// The pagination details
    fn from_query(query: Query<'q>) -> Result<Self, Self::Error> {
        let mut result = Self::default();

        for item in query {
            if item.key == "offset" {
                if let Ok(value) = u32::from_str(item.value.as_str()) {
                    result.offset = value;
                }
            } else if item.key == "count" {
                if let Ok(value) = u32::from_str(item.value.as_str()) {
                    result.count = value;
                }
            }
        }

        Ok(result)
    }
}
