
/// The response from a test request to the server
pub struct TestResponse<'r> {
    response: rocket::local::LocalResponse<'r>,
}

impl<'r> TestResponse<'r> {
    /// Build the headers of the response
    ///
    /// # Returns
    /// A string representing the header section of the response
    pub fn headers(&self) -> String {
        let headers = self
            .response
            .headers()
            .iter()
            .map(|header| format!("{}: {}", header.name(), header.value()))
            .collect::<Vec<String>>()
            .join("\n");

        format!("HTTP/1.1 {}\n{}", self.response.status(), headers)
    }

    /// Convert the response body to a String
    ///
    /// # Returns
    /// The body of the response
    pub fn to_string(&mut self) -> String {
        self.response.body_string().unwrap_or_else(|| "".to_owned())
    }

    /// Convert the response body to JSON
    ///
    /// # Returns
    /// The body of the response, converted to a Serde JSON object
    ///
    /// # Errors
    /// Any errors from deserializing the response
    pub fn to_json(&mut self) -> Result<serde_json::Value, serde_json::error::Error> {
        let body = self.response.body_string().unwrap_or_else(|| "".to_owned());
        serde_json::from_str(&body)
    }
}

impl<'r> From<rocket::local::LocalResponse<'r>> for TestResponse<'r> {
    fn from(response: rocket::local::LocalResponse<'r>) -> TestResponse<'r> {
        Self { response }
    }
}
