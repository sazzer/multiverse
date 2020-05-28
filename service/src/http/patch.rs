use serde::{Deserialize, Deserializer};

/// Enumeration of possible states for each field in an HTTP Patch
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Patch<T> {
    /// The field was missing - i.e. it was undefined
    Missing,
    // The field was present but didn't have a value
    Null,
    /// The field was present and had a value
    Value(T),
}

impl<T> Patch<T> {
    /// Map the value inside the Patch to a different value, possibly of a different type.
    ///
    /// A `Missing` or `Null` value is left as-is.
    /// A `Value(v)` will get transformed into the new value
    ///
    /// # Parameters
    /// - `f` - The function to transform the value
    ///
    /// # Returns
    /// The transformed value
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Patch<U> {
        match self {
            Patch::Missing => Patch::Missing,
            Patch::Null => Patch::Null,
            Patch::Value(v) => Patch::Value(f(v)),
        }
    }

    /// Filter the value inside the Patch, returning `Null` if the value fails the predicate
    ///
    /// A `Missing` or `Null` value is left as-is.
    /// A `Value(v)` will get transformed into the new value
    ///
    /// # Parameters
    /// - `predicate` - The function to evaluate the value
    ///
    /// # Returns
    /// The filtered value
    pub fn filter_null<P: FnOnce(&T) -> bool>(self, predicate: P) -> Self {
        match self {
            Patch::Missing => Patch::Missing,
            Patch::Null => Patch::Null,
            Patch::Value(v) => {
                if predicate(&v) {
                    Patch::Value(v)
                } else {
                    Patch::Null
                }
            }
        }
    }

    /// Convert the value to an `Option<T>` but wrapped in a `Result`.
    /// The `Result` returns `Err(err)` if the `Patch` is `Null`, otherwise returns an `Ok(Option<T>)`
    /// containing the value.
    ///
    /// # Parameters
    /// - `err` - The error value to use
    ///
    /// # Returns
    /// - `Patch::Value(v)` -> `Ok(Some(v))`
    /// - `Patch::Missing` -> `Ok(None`
    /// - `Patch::Null` -> `Err(err)`
    pub fn ok_not_null<E>(self, err: E) -> Result<Option<T>, E> {
        match self {
            Patch::Null => Err(err),
            Patch::Missing => Ok(None),
            Patch::Value(v) => Ok(Some(v)),
        }
    }
}

impl<T, E> Patch<Result<T, E>> {
    /// Transpose a patch containing a result into a result containing the patch
    ///
    /// # Returns
    /// - `Patch::Missing` -> Ok(Missing)
    /// - `Patch::Null` -> Ok(Null)
    /// - `Patch::Value(Ok(v)) -> Ok(Value(v))
    /// - `Patch::Value(Err(e)) -> Err(e)
    pub fn transpose(self) -> Result<Patch<T>, E> {
        match self {
            Patch::Missing => Ok(Patch::Missing),
            Patch::Null => Ok(Patch::Null),
            Patch::Value(Ok(v)) => Ok(Patch::Value(v)),
            Patch::Value(Err(e)) => Err(e),
        }
    }
}

impl<T> Default for Patch<T> {
    fn default() -> Self {
        Patch::Missing
    }
}

impl<T> From<Option<T>> for Patch<T> {
    fn from(opt: Option<T>) -> Patch<T> {
        match opt {
            Some(v) => Patch::Value(v),
            None => Patch::Null,
        }
    }
}

impl<'de, T> Deserialize<'de> for Patch<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use galvanic_assert::{assert_that, matchers::*};

    #[test]
    fn test_map() {
        assert_that!(&Patch::Missing.map(|v: u32| v), eq(Patch::Missing));
        assert_that!(&Patch::Null.map(|v: u32| v), eq(Patch::Null));
        assert_that!(&Patch::Value(5).map(|v| v * 2), eq(Patch::Value(10)));
        assert_that!(
            &Patch::Value(5).map(|v| format!("{}", v)),
            eq(Patch::Value("5".to_owned()))
        );
    }

    #[test]
    fn test_filter_null() {
        assert_that!(
            &Patch::Missing.filter_null(|_: &u32| true),
            eq(Patch::Missing)
        );
        assert_that!(
            &Patch::Missing.filter_null(|_: &u32| false),
            eq(Patch::Missing)
        );
        assert_that!(&Patch::Null.filter_null(|_: &u32| true), eq(Patch::Null));
        assert_that!(&Patch::Null.filter_null(|_: &u32| false), eq(Patch::Null));
        assert_that!(&Patch::Value(1).filter_null(|_| true), eq(Patch::Value(1)));
        assert_that!(&Patch::Value(1).filter_null(|_| false), eq(Patch::Null));
    }

    #[test]
    fn test_ok_not_null() {
        assert_that!(&Patch::<u32>::Missing.ok_not_null(5), eq(Ok(None)));
        assert_that!(&Patch::<u32>::Null.ok_not_null(5), eq(Err(5)));
        assert_that!(&Patch::<u32>::Value(1).ok_not_null(5), eq(Ok(Some(1))));
    }

    #[test]
    fn test_transpose() {
        assert_that!(
            &Patch::<Result<u32, u32>>::Missing.transpose(),
            eq(Ok(Patch::Missing))
        );
        assert_that!(
            &Patch::<Result<u32, u32>>::Null.transpose(),
            eq(Ok(Patch::Null))
        );
        assert_that!(
            &Patch::<Result<u32, u32>>::Value(Ok(1)).transpose(),
            eq(Ok(Patch::Value(1)))
        );
        assert_that!(
            &Patch::<Result<u32, u32>>::Value(Err(5)).transpose(),
            eq(Err(5))
        );
    }
}
