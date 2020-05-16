use super::Server;
use actix_web::http::{HeaderMap, StatusCode};
use bytes::Bytes;
use serde_json::Value;

/// The response from a test request to the server
#[derive(Clone)]
pub struct TestResponse {
    /// The status code from the response
    pub status: StatusCode,
    /// The headers from the response
    pub headers: HeaderMap,
    /// The actual body from the response
    pub body: Bytes,
}

impl TestResponse {
    /// Build the headers of the response
    ///
    /// # Returns
    /// A string representing the header section of the response
    pub fn headers(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap()))
            .collect::<Vec<String>>()
            .join("\n");

        format!("HTTP/1.1 {}\n{}", self.status, headers)
    }

    /// Convert the response body to JSON
    ///
    /// # Returns
    /// The body of the response, converted to a Serde JSON object
    ///
    /// # Errors
    /// Any errors from deserializing the response
    pub fn to_json(&self) -> Result<Value, serde_json::error::Error> {
        serde_json::from_slice(&self.body)
    }
}

impl Server {
    /// Send an HTTP Request in to the service and return the response
    ///
    /// # Parameters
    /// - `request` - The request to send to the service
    ///
    /// # Returns
    /// The HTTP Response
    pub fn test_request(&self, request: actix_http::Request) -> TestResponse {
        todo!()
    }
}
