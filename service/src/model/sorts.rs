use std::str::FromStr;

/// Enumeration of the directions in which we can sort by a given field
#[derive(Debug, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    Default,
}

/// Individual field that can be sorted on
#[derive(Debug, PartialEq)]
pub struct SortField<T> {
    /// The actual field to sort on
    pub field: T,
    /// The direction in which to sort
    pub direction: SortDirection,
}

/// Set of fields that can be sorted on
#[derive(Debug, PartialEq)]
pub struct SortFields<T>(pub Vec<SortField<T>>);

impl<T> SortFields<T> {
    pub fn iter(&self) -> impl std::iter::Iterator<Item = &SortField<T>> {
        self.0.iter()
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseSortFieldError {
    #[error("The specified field was unknown: {0}")]
    UnknownField(String),
}

impl<T> FromStr for SortField<T>
where
    T: FromStr,
    T: strum::VariantNames,
    T: std::fmt::Debug,
{
    type Err = ParseSortFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, field_name) = match s.get(..1) {
            Some("+") => (SortDirection::Ascending, &s[1..]),
            Some("-") => (SortDirection::Descending, &s[1..]),
            _ => (SortDirection::Default, s),
        };

        let field_name = field_name.to_lowercase().trim().to_owned();

        let field = T::VARIANTS
            .iter()
            .find(|v| v.to_lowercase().clone() == field_name)
            .and_then(|v| T::from_str(v).ok())
            .map(|field| SortField { field, direction })
            .ok_or(ParseSortFieldError::UnknownField(s.to_owned()));

        tracing::debug!(input = s, result = ?field, "Parse sort field");
        field
    }
}

impl<T> FromStr for SortFields<T>
where
    T: FromStr,
    T: strum::VariantNames,
    T: std::fmt::Debug,
{
    type Err = ParseSortFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split(",")
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| SortField::<T>::from_str(v))
            .collect::<Result<Vec<SortField<T>>, Self::Err>>()
            .map(|fields| SortFields(fields));

        tracing::debug!(input = s, result = ?fields, "Parse sort fields");
        fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use galvanic_assert::{
        assert_that,
        matchers::{variant::*, *},
    };

    #[derive(Debug, PartialEq, strum_macros::EnumVariantNames, strum_macros::EnumString)]
    pub enum TestSortField {
        Name,
        Created,
        Updated,
    }

    #[test]
    fn test_parse_default_sort_field() {
        let result = SortField::<TestSortField>::from_str("name");

        assert_that!(
            &result,
            maybe_ok(eq(SortField {
                field: TestSortField::Name,
                direction: SortDirection::Default
            }))
        );
    }

    #[test]
    fn test_parse_ascending_sort_field() {
        let result = SortField::<TestSortField>::from_str("+created");

        assert_that!(
            &result,
            maybe_ok(eq(SortField {
                field: TestSortField::Created,
                direction: SortDirection::Ascending
            }))
        );
    }

    #[test]
    fn test_parse_descending_sort_field() {
        let result = SortField::<TestSortField>::from_str("-updated");

        assert_that!(
            &result,
            maybe_ok(eq(SortField {
                field: TestSortField::Updated,
                direction: SortDirection::Descending
            }))
        );
    }

    #[test]
    fn test_parse_unknown_sort_field() {
        let result = SortField::<TestSortField>::from_str("unknown");

        assert_that!(
            &result,
            maybe_err(eq(ParseSortFieldError::UnknownField("unknown".to_owned())))
        );
    }

    #[test]
    fn test_parse_unknown_ascending_sort_field() {
        let result = SortField::<TestSortField>::from_str("+unknown");

        assert_that!(
            &result,
            maybe_err(eq(ParseSortFieldError::UnknownField("+unknown".to_owned())))
        );
    }

    #[test]
    fn test_parse_single_sort_fields() {
        let result = SortFields::<TestSortField>::from_str("name");

        assert_that!(
            &result,
            maybe_ok(eq(SortFields(vec![SortField {
                field: TestSortField::Name,
                direction: SortDirection::Default
            }])))
        );
    }

    #[test]
    fn test_parse_multiple_sort_fields() {
        let result = SortFields::<TestSortField>::from_str("name,+created,-updated");

        assert_that!(
            &result,
            maybe_ok(eq(SortFields(vec![
                SortField {
                    field: TestSortField::Name,
                    direction: SortDirection::Default
                },
                SortField {
                    field: TestSortField::Created,
                    direction: SortDirection::Ascending
                },
                SortField {
                    field: TestSortField::Updated,
                    direction: SortDirection::Descending
                }
            ])))
        );
    }

    #[test]
    fn test_parse_sort_fields_one_unknown() {
        let result = SortFields::<TestSortField>::from_str("name,+creates,-updated");

        assert_that!(
            &result,
            maybe_err(eq(ParseSortFieldError::UnknownField("+creates".to_owned())))
        );
    }

    #[test]
    fn test_parse_multiple_sort_fields_whitespace() {
        let result = SortFields::<TestSortField>::from_str("name , +created , -updated");

        assert_that!(
            &result,
            maybe_ok(eq(SortFields(vec![
                SortField {
                    field: TestSortField::Name,
                    direction: SortDirection::Default
                },
                SortField {
                    field: TestSortField::Created,
                    direction: SortDirection::Ascending
                },
                SortField {
                    field: TestSortField::Updated,
                    direction: SortDirection::Descending
                }
            ])))
        );
    }

    #[test]
    fn test_parse_multiple_sort_fields_blanks() {
        let result = SortFields::<TestSortField>::from_str("name,,+created,,-updated");

        assert_that!(
            &result,
            maybe_ok(eq(SortFields(vec![
                SortField {
                    field: TestSortField::Name,
                    direction: SortDirection::Default
                },
                SortField {
                    field: TestSortField::Created,
                    direction: SortDirection::Ascending
                },
                SortField {
                    field: TestSortField::Updated,
                    direction: SortDirection::Descending
                }
            ])))
        );
    }
}
