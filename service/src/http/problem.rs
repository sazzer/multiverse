use actix_web::http::StatusCode;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

mod response;

/// Trait to represent the type of a problem
pub trait ProblemType: Display + Debug {
    /// Generate a Type value for the `ProblemType` values.
    ///
    /// These are used in the `type` field in the RFC-7807 Problem Response
    fn error_code(&self) -> &'static str;
}

/// Representation of a Problem expressed in terms of RFC-7807
#[derive(Debug)]
pub struct Problem<T> {
    pub error: T,
    pub status: StatusCode,
    pub detail: Option<String>,
    pub instance: Option<String>,
    pub extra: HashMap<String, Value>,
}

impl<T> Problem<T>
where
    T: ProblemType,
{
    /// Create a new Problem instance
    pub fn new(error: T, status: StatusCode) -> Self {
        Self {
            error,
            status,
            detail: None,
            instance: None,
            extra: HashMap::new(),
        }
    }

    /// Set the Detail of the Problem instance
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

impl<T> Display for Problem<T>
where
    T: ProblemType,
{
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
        let problem = Problem::new(ProblemDetails::SomeProblem, StatusCode::BAD_REQUEST);

        assert_eq!(StatusCode::BAD_REQUEST, problem.status);
        assert_eq!(ProblemDetails::SomeProblem, problem.error);
        assert_eq!(None, problem.detail);
        assert_eq!(None, problem.instance);
        assert_eq!(0, problem.extra.len());
    }

    #[test]
    fn test_full_problem() {
        let problem = Problem::new(ProblemDetails::SomeProblem, StatusCode::BAD_REQUEST)
            .with_detail("Some Detail")
            .with_instance("Some Instance")
            .with_extra("some_key", "Some Value")
            .with_extra("other_key", 42);

        assert_eq!(StatusCode::BAD_REQUEST, problem.status);
        assert_eq!(ProblemDetails::SomeProblem, problem.error);
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
        let problem = Problem::new(ProblemDetails::SomeProblem, StatusCode::BAD_REQUEST);

        assert_eq!("Something Happened".to_owned(), format!("{}", problem));
    }
}
