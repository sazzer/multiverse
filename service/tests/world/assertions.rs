use crate::World;
use actix_web::http::{header::HeaderValue, StatusCode};
use galvanic_assert::{
    assert_that,
    matchers::{variant::*, *},
};
use multiverse_lib::TestResponse;
use serde_json::{json, Value};

impl World {
    /// Assert that a value has indeede been received from the server.
    ///
    /// This will pass if and only if a request has been made and a response received in this scenario
    ///
    /// # Returns
    /// The response that was received, if there was one
    pub fn assert_received(&self) -> TestResponse {
        let received = self.last_response();
        assert_that!(&received, maybe_some(any_value()));

        received.unwrap()
    }

    /// Assert that the last response received has the status code given
    ///
    /// # Parameters
    /// - `status_code` - The status code we expect to have received
    pub fn assert_status_code(&self, status_code: StatusCode) {
        let response = self.assert_received();
        assert_that!(&response.status, eq(status_code));
    }

    /// Assert that the given value exists at the given path into the response
    ///
    /// # Parameters
    /// - `path` - The path into the JSON response to look
    /// - `value` - The expected value
    pub fn assert_json_value<S>(&self, path: S, value: Value)
    where
        S: Into<String>,
    {
        let extracted_value = self.extract_response_value(path);
        assert_that!(&extracted_value, maybe_some(eq(value)));
    }

    /// Assert that the given response header was present and had the given vale
    ///
    /// # Parameters
    /// - `header` - The name of the header to assert on
    /// - `value` - The expected value of the header
    pub fn assert_header<H, V>(&self, header: H, value: V)
    where
        H: Into<String>,
        V: Into<String>,
    {
        let desired_value = HeaderValue::from_str(value.into().as_ref()).unwrap();

        let response = self.assert_received();
        let header_value = response.headers.get(header.into());

        assert_that!(&header_value, maybe_some(eq(&desired_value)));
    }

    /// Assert that the response was an RFC-7807 Problem with the given Status Code and Problem Type
    ///
    /// # Parameters
    /// - `status_code` - The status code to assert
    /// - `problem_type` - The problem type to assert - literally the value of the "type" JSON field
    pub fn assert_problem<S>(&self, status_code: StatusCode, problem_type: S)
    where
        S: Into<String>,
    {
        self.assert_status_code(status_code);
        self.assert_header("content-type", "application/problem+json");
        self.assert_json_value("$.type", json!(problem_type.into()));
        self.assert_json_value("$.status", json!(status_code.as_u16()));
    }
}
