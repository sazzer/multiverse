use rocket::http::Status;
use serde::Serialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};
pub use validation::*;

mod response;
mod validation;

/// Trait to represent the type of a problem
pub trait ProblemType: Display + Debug {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str;
}

/// Representation of a Problem expressed in terms of RFC-7807
#[derive(Debug)]
pub struct Problem {
    /// The actual error that occurred
    pub error: Box<dyn ProblemType>,
    /// The HTTP Status code to use
    pub status: Status,
    /// An additional detail message
    pub detail: Option<String>,
    /// An additional instance subtype
    pub instance: Option<String>,
    /// Any extra details
    pub extra: HashMap<String, Value>,
}

impl Problem {
    /// Create a new Problem instance
    ///
    /// # Parameters
    /// - `error` - The error code
    /// - `status` - The HTTP Status code
    ///
    /// # Returns
    /// The problem
    pub fn new<T>(error: T, status: Status) -> Self
    where
        T: ProblemType + 'static,
    {
        Self {
            error: Box::new(error),
            status,
            detail: None,
            instance: None,
            extra: HashMap::new(),
        }
    }

    /// Set the Detail of the Problem instance
    #[allow(dead_code)]
    pub fn with_detail<S>(self, detail: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            detail: Some(detail.into()),
            ..self
        }
    }

    /// Set the Instance of the Problem instance
    #[allow(dead_code)]
    pub fn with_instance<S>(self, instance: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            instance: Some(instance.into()),
            ..self
        }
    }

    /// Set some extra data on the Problem instance
    pub fn with_extra<K, V>(self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        let mut extra = self.extra;
        extra.insert(
            key.into(),
            serde_json::to_value(value).expect("Failed to serialize extra detail"),
        );

        Self { extra, ..self }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(thiserror::Error, Debug, PartialEq)]
    pub enum ProblemDetails {
        #[error("Something Happened")]
        SomeProblem,
    }

    impl ProblemType for ProblemDetails {
        fn error_code(&self) -> &'static str {
            "tag:spacegame,2020:some/problem"
        }
    }

    #[test]
    fn test_basic_problem() {
        let problem = Problem::new(ProblemDetails::SomeProblem, Status::BadRequest);

        assert_eq!(Status::BadRequest, problem.status);
        // TODO: assert_matches!(*problem.error, ProblemDetails::SomeProblem);
        assert_eq!(None, problem.detail);
        assert_eq!(None, problem.instance);
        assert_eq!(0, problem.extra.len());
    }

    #[test]
    fn test_full_problem() {
        let problem = Problem::new(ProblemDetails::SomeProblem, Status::BadRequest)
            .with_detail("Some Detail")
            .with_instance("Some Instance")
            .with_extra("some_key", "Some Value")
            .with_extra("other_key", 42);

        assert_eq!(Status::BadRequest, problem.status);
        // TODO: assert_matches!(*problem.error, ProblemDetails::SomeProblem);
        assert_eq!(Some("Some Detail".to_owned()), problem.detail);
        assert_eq!(Some("Some Instance".to_owned()), problem.instance);
        assert_eq!(2, problem.extra.len());
        assert_eq!(
            Some(&serde_json::to_value("Some Value").unwrap()),
            problem.extra.get(&"some_key".to_owned())
        );
        assert_eq!(
            Some(&serde_json::to_value(42).unwrap()),
            problem.extra.get(&"other_key".to_owned())
        );
    }

    #[test]
    fn test_problem_display() {
        let problem = Problem::new(ProblemDetails::SomeProblem, Status::BadRequest);

        assert_eq!("Something Happened".to_owned(), format!("{}", problem));
    }
}
