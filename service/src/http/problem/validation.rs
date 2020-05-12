use super::{Problem, ProblemType};
use actix_web::http::StatusCode;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

/// Trait to represent the type of validation error
pub trait ValidationProblemType: Display + Debug {
    /// Generate a Type value for the `ValidationProblemType` values.
    fn error_code(&self) -> &'static str;
}

#[derive(Debug, thiserror::Error)]
pub enum GenericUserValidationProblem {
    /// The required field was missing
    #[error("The required field was missing")]
    Missing,
}

impl ValidationProblemType for GenericUserValidationProblem {
    /// Generate a Type value for the `ValidationProblemType` values.
    fn error_code(&self) -> &'static str {
        match self {
            GenericUserValidationProblem::Missing => {
                "tag:multiverse,2020:problems/validation_error/missing"
            }
        }
    }
}

/// Builder to help build an RFC-7807 Problem response representing a Validation error
#[derive(Debug)]
pub struct ValidationProblem<P>
where
    P: ProblemType,
{
    problem_type: P,
    fields: HashMap<String, Box<dyn ValidationProblemType>>,
}

#[derive(Serialize)]
pub struct ValidationProblemModel {
    /// The Type field in the response
    r#type: &'static str,
    /// The Title field in the response
    title: String,
}

impl<P> ValidationProblem<P>
where
    P: ProblemType,
{
    /// Construct a Validation Problem
    ///
    /// # Parameters
    /// - `problem_type` - The type of problem to return
    ///
    /// # Returns
    /// The Validation Problem used to build the actual RFC-7807 Problem response
    pub fn new(problem_type: P) -> Self {
        ValidationProblem {
            problem_type,
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
        V: ValidationProblemType + 'static,
    {
        self.fields.insert(field.into(), Box::new(error));
        self
    }

    /// Actually build the RFC-7807 Problem to respond to the client with
    ///
    /// # Returns
    /// The RFC-7807 Problem to send to the client
    pub fn build(self) -> Problem<P> {
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

        Problem::new(self.problem_type, StatusCode::UNPROCESSABLE_ENTITY)
            .with_extra("fields", fields)
    }
}
