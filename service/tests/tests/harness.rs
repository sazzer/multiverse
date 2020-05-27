use crate::service::{Seedable, TestService};
use pretty_assertions::assert_eq;
use rocket::{
    http::{Header, Status},
    local::{Client, LocalResponse},
};
use serde_json::Value;
use std::collections::HashMap;

/// The test harness with which we can write our integration tests
pub struct TestHarness {
    /// The service being tested
    service: TestService,
    /// The HTTP Client to interact with the service
    client: Client,
    /// The authentication token to use
    authentication_token: Option<String>,
    /// The last HTTP response received
    last_response: Option<Response>,
}

/// Representation of an HTTP Response
pub struct Response {
    /// The status code
    pub status: Status,
    /// The headers
    pub headers: HashMap<String, String>,
    /// The body
    pub body: String,
}

impl<'r> From<LocalResponse<'r>> for Response {
    fn from(mut response: LocalResponse) -> Self {
        let headers = response
            .headers()
            .iter()
            .map(|h| (h.name().to_owned(), h.value().to_owned()))
            .collect();

        let status = response.status();

        let body = response.body_string().unwrap_or_else(|| "".to_owned());

        Self {
            status,
            headers,
            body,
        }
    }
}

/// Start a test harness running
pub fn run_test() -> TestHarness {
    let service = TestService::new();
    let client = service.test_client();

    TestHarness {
        service,
        client,
        authentication_token: None,
        last_response: None,
    }
}

impl TestHarness {
    /// Add some seed data into the database being used for this test
    ///
    /// # Parameters
    /// - `data` - The data to add
    ///
    /// # Returns
    /// Self, for chaining
    pub fn seed<D>(self, data: D) -> Self
    where
        D: Seedable,
    {
        self.service.seed(data);

        self
    }

    /// Authenticate the requests as the given username and password
    ///
    /// # Parameters
    /// - `username` - The username to authenticate as
    /// - `password` - The password to authenticate as
    ///
    /// # Returns
    /// Self, for chaining
    pub fn authenticate<U, P>(self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        let client = self.client;
        let body = serde_json::json!({
            "username": username.into(),
            "password": password.into()
        });
        let response: Response = client
            .post("/login")
            .body(serde_json::to_string(&body).unwrap())
            .dispatch()
            .into();
        assert_eq!(response.status, Status::Ok);
        let response_body: Value = serde_json::from_str(&response.body).unwrap();
        let token = response_body
            .pointer("/token")
            .and_then(|v| v.as_str())
            .unwrap()
            .to_owned();

        Self {
            client,
            authentication_token: Some(token),
            ..self
        }
    }

    /// Make a GET request to the service
    ///
    /// # Parameters
    /// - `url` - The URL to make the request to
    ///
    /// # Returns
    /// Self, for chaining
    pub fn get<S>(self, url: S) -> Self
    where
        S: Into<String>,
    {
        let client = self.client;
        let mut request = client.get(url.into());
        if let Some(token) = &self.authentication_token {
            request = request.header(Header::new("Authorization", format!("Bearer {}", token)));
        }
        let response = request.dispatch().into();

        Self {
            client,
            last_response: Some(response),
            ..self
        }
    }

    /// Make a POST request to the service
    ///
    /// # Parameters
    /// - `url` - The URL to make the request to
    /// - `body` - The JSON Body to make the request with
    ///
    /// # Returns
    /// Self, for chaining
    pub fn post<S, B>(self, url: S, body: B) -> Self
    where
        S: Into<String>,
        B: Into<Value>,
    {
        let client = self.client;
        let mut request = client
            .post(url.into())
            .body(serde_json::to_string(&body.into()).unwrap());
        if let Some(token) = &self.authentication_token {
            request = request.header(Header::new("Authorization", format!("Bearer {}", token)));
        }
        let response = request.dispatch().into();

        Self {
            client,
            last_response: Some(response),
            ..self
        }
    }

    /// Make a PATCH request to the service
    ///
    /// # Parameters
    /// - `url` - The URL to make the request to
    /// - `body` - The JSON Body to make the request with
    ///
    /// # Returns
    /// Self, for chaining
    pub fn patch<S, B>(self, url: S, body: B) -> Self
    where
        S: Into<String>,
        B: Into<Value>,
    {
        let client = self.client;
        let mut request = client
            .patch(url.into())
            .body(serde_json::to_string(&body.into()).unwrap());
        if let Some(token) = &self.authentication_token {
            request = request.header(Header::new("Authorization", format!("Bearer {}", token)));
        }
        let response = request.dispatch().into();

        Self {
            client,
            last_response: Some(response),
            ..self
        }
    }

    /// Assert that we have a response and that the response has the expected status code
    ///
    /// # Parameters
    /// - `status` - The status code to expect
    ///
    /// # Returns
    /// Self, for chaining
    pub fn has_status(self, status: Status) -> Self {
        self.assert_response(|response| {
            assert_eq!(response.status, status);
        })
    }

    /// Assert that we have a response and that the response has the expected header
    ///
    /// # Parameters
    /// - `header` - The name of the header
    /// - `value` - The desired value of the header
    ///
    /// # Returns
    /// Self, for chaining
    pub fn has_header<H, V>(self, header: H, value: V) -> Self
    where
        H: Into<String>,
        V: Into<String>,
    {
        self.assert_response(|response| {
            let expected_value = value.into();
            let header_value = response.headers.get(&header.into());
            assert_eq!(header_value, Some(&expected_value));
        })
    }

    /// Assert that we have a response, that the response has a body that is parsable as JSON and that
    /// this JSON is what we expected
    ///
    /// # Parameters
    /// - `body` - The expected body
    ///
    /// # Returns
    /// Self, for chaining
    pub fn has_json_body<B>(self, body: B) -> Self
    where
        B: Into<Value>,
    {
        self.assert_json_body(|actual_body| {
            let expected_body = body.into();

            let actual_body_string = serde_json::to_string(actual_body).unwrap();
            let expected_body_string = serde_json::to_string(&expected_body).unwrap();
            assert_eq!(actual_body_string, expected_body_string);
        })
    }

    /// Assert some details about the response are as desired
    ///
    /// # Parameters
    /// - `f` - The function to assert the response with
    ///
    /// # Returns
    /// Self, for chaining
    pub fn assert_response<F>(self, f: F) -> Self
    where
        F: FnOnce(&Response),
    {
        assert!(self.last_response.is_some());
        if let Some(response) = &self.last_response {
            f(response);
        }
        self
    }

    /// Assert some details about the body, having already parsed it as JSON
    ///
    /// # Parameters
    /// - `f` - The function to assert the response with
    ///
    /// # Returns
    /// Self, for chaining
    pub fn assert_json_body<F>(self, f: F) -> Self
    where
        F: FnOnce(&Value),
    {
        self.assert_response(|response| {
            let actual_body = serde_json::from_str(&response.body).unwrap();
            f(&actual_body);
        })
    }

    pub fn assert_database<F>(self, f: F) -> Self
    where
        F: FnOnce(postgres::Transaction),
    {
        let mut connection = self.service.database_connection();
        let transaction: postgres::Transaction = connection.transaction().unwrap();

        f(transaction);

        self
    }
}
