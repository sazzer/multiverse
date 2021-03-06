use super::Problem;
use rocket::{http::ContentType, response, Request};
use rocket_contrib::json::Json;
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

impl<'r> response::Responder<'r> for Problem {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let problem_details = ProblemModel {
            r#type: self.error.error_code(),
            title: format!("{}", self.error),
            status: self.status.code,
            detail: self.detail.clone(),
            instance: self.instance.clone(),
            extra: self.extra.clone(),
        };

        response::Response::build()
            .merge(Json(problem_details).respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::new("application", "problem+json"))
            .ok()
    }
}
