use super::{Problem, ProblemType};
use rocket::http::Status;
use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

/// Trait to represent the type of validation error
pub trait ValidationType: Display + Debug {
    /// Generate a Type value for the `ValidationType` values.
    fn error_code(&self) -> &'static str;
}

#[derive(Debug, thiserror::Error)]
pub enum GenericValidation {
    /// The required field was missing
    #[error("The required field was missing")]
    Missing,
}

impl ValidationType for GenericValidation {
    /// Generate a Type value for the `ValidationType` values.
    fn error_code(&self) -> &'static str {
        match self {
            GenericValidation::Missing => "tag:multiverse,2020:problems/validation_error/missing",
        }
    }
}

/// Builder to help build an RFC-7807 Problem response representing a Validation error
#[derive(Debug)]
pub struct ValidationProblem {
    fields: HashMap<String, Box<dyn ValidationType>>,
}

/// API Model to represent any Validation Problem details
#[derive(Serialize)]
pub struct ValidationProblemModel {
    /// The Type field in the response
    r#type: &'static str,
    /// The Title field in the response
    title: String,
}

impl ValidationProblem {
    /// Construct a Validation Problem
    ///
    /// # Parameters
    /// - `problem_type` - The type of problem to return
    ///
    /// # Returns
    /// The Validation Problem used to build the actual RFC-7807 Problem response
    pub fn new() -> Self {
        ValidationProblem {
            fields: HashMap::new(),
        }
    }

    /// Register a new field error to report in the response
    ///
    /// # Paremeters
    /// - `field` - The field that was in error
    /// - `error` - The error that the field was in
    ///
    /// # Returns
    /// Self, for chaining if needed
    pub fn with_field_error<S, V>(&mut self, field: S, error: V) -> &mut Self
    where
        S: Into<String>,
        V: ValidationType + 'static,
    {
        self.fields.insert(field.into(), Box::new(error));
        self
    }

    /// Actually build the RFC-7807 Problem to respond to the client with
    ///
    /// # Returns
    /// The RFC-7807 Problem to send to the client
    pub fn build(self) -> Problem {
        let fields: HashMap<String, ValidationProblemModel> = self
            .fields
            .into_iter()
            .map(|(field, value)| {
                let model = ValidationProblemModel {
                    r#type: value.error_code(),
                    title: format!("{}", value),
                };
                (field, model)
            })
            .collect();

        Problem::new(ValidationProblemType {}, Status::UnprocessableEntity)
            .with_extra("fields", fields)
    }
}

#[derive(Debug)]
struct ValidationProblemType {}

impl Display for ValidationProblemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occurred")
    }
}

impl ProblemType for ValidationProblemType {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str {
        "tag:multiverse,2020:problems/validation_error"
    }
}
