mod service;
mod json;

use multiverse_lib::TestResponse;
use service::TestService;
use std::sync::Arc;

pub struct World {
    service: Arc<TestService>,
    last_response: Option<TestResponse>,
}

impl cucumber_rust::World for World {}

impl Default for World {
    /// Instantiate the World, starting up the actual service that we're testing against
    ///
    /// # Returns
    /// The World, ready for testing with
    fn default() -> Self {
        let mut rt = actix_rt::Runtime::new().unwrap();

        let service = rt.block_on(async {
            TestService::new().await
        });

        Self { service: Arc::new(service), last_response: None }
    }
}

impl World {
    /// Make an HTTP Request to the test service and await a response
    ///
    /// # Parameters
    /// - `request` - The HTTP Request to make
    pub fn request(&mut self, request: actix_http::Request) {
        let span = tracing::warn_span!("cucumber_request");
        let _enter = span.enter();
        tracing::debug!(request = ?request, "Making request");
        self.last_response = None;

        let mut rt = actix_rt::Runtime::new().unwrap();
        let service = self.service.clone();
        let response = rt.block_on(async move {
            service
                .request(request)
                .await
        });

        tracing::debug!(response = ?response.status, "Received response");
        self.last_response = Some(response);
    }

    /// Get the last response from the service, if there was one
    ///
    /// # Returns
    /// The last response that was received from the service, or `None` if no response was received yet
    pub fn last_response(&self) -> Option<TestResponse> {
        self.last_response.clone()
    }
}
