use super::Server;
use actix_cors::Cors;
use actix_web::http::{HeaderMap, StatusCode};
use actix_web::{middleware::Logger, App};
use bytes::Bytes;
use serde_json::Value;
use std::ops::Deref;

/// The response from a test request to the server
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
    pub async fn test_request(&self, request: actix_http::Request) -> TestResponse {
        let mut app = App::new()
            .wrap(Logger::default())
            .wrap(Cors::new().finish());
        for config in &self.configs {
            app = app.configure(config.deref());
        }

        let mut app = actix_web::test::init_service(app).await;
        let response = actix_web::test::call_service(&mut app, request).await;

        let status = response.status();
        let headers = response.headers().clone();
        let body = actix_web::test::read_body(response).await;
        TestResponse {
            status,
            headers,
            body,
        }
    }
}
