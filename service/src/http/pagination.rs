use crate::model::Pagination;
use rocket::request::{FromQuery, Query};
use std::{ops::Deref, str::FromStr};

/// Request details to describe the pagination of a request
#[derive(Debug)]
pub struct PaginationRequest(Pagination);

impl Deref for PaginationRequest {
    type Target = Pagination;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        let mut result = Pagination::default();

        for item in query {
            if item.key == "offset" {
                if let Ok(value) = u64::from_str(item.value.as_str()) {
                    result.offset = value;
                }
            } else if item.key == "count" {
                if let Ok(value) = u64::from_str(item.value.as_str()) {
                    result.count = value;
                }
            }
        }

        Ok(PaginationRequest(result))
    }
}
