use super::Problem;
use actix_web::{
    http::{header, StatusCode},
    Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::future::{ready, Ready};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// API Model representing an RFC-7807 Problem
#[derive(Serialize)]
struct ProblemModel {
    /// The Type field in the response
    r#type: &'static str,
    /// The Title field in the response
    title: String,
    /// The Status field in the response
    status: u16,
    /// The Detail field in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    /// The Instance field in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,
    /// Any additional fields in the response
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl From<&Problem> for HttpResponse {
    fn from(problem: &Problem) -> HttpResponse {
        let problem_details = ProblemModel {
            r#type: problem.error.error_code(),
            title: format!("{}", problem.error),
            status: problem.status.as_u16(),
            detail: problem.detail.clone(),
            instance: problem.instance.clone(),
            extra: problem.extra.clone(),
        };

        HttpResponse::build(problem.status)
            .header(header::CONTENT_TYPE, "application/problem+json")
            .json(problem_details)
    }
}

impl From<Problem> for HttpResponse {
    fn from(problem: Problem) -> HttpResponse {
        HttpResponse::from(&problem)
    }
}

impl Responder for Problem {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // Create response and set content type
        ready(Ok(self.into()))
    }
}

impl ResponseError for Problem {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::super::ProblemType;
    use super::*;

    #[derive(thiserror::Error, Debug, PartialEq)]
    pub enum ProblemDetails {
        #[error("Something Happened")]
        SomeProblem,
    }

    impl ProblemType for ProblemDetails {
        fn error_code(&self) -> &'static str {
            "tag:multiverse,2020:some/problem"
        }
    }

    #[test]
    fn test_basic_problem_to_response() {
        let problem = Problem::new(ProblemDetails::SomeProblem, StatusCode::BAD_REQUEST);

        let response: HttpResponse = problem.into();
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
        assert_eq!(
            "application/problem+json",
            response.headers().get(header::CONTENT_TYPE).unwrap()
        );
        // TODO: Assert response body
    }
}
