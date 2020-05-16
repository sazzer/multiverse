use postgres::error::{DbError, SqlState};

/// Errors that can occur when saving a user record
#[derive(Debug, thiserror::Error)]
pub enum SaveUserError {
    /// An unknown error occurred
    #[error("An unknown error occurred")]
    UnknownError,

    /// The username is already registered
    #[error("The username is already registered")]
    DuplicateUsername,
}

impl From<postgres::Error> for SaveUserError {
    /// Convert a database error into a SaveUserError.
    ///
    /// The conversion returns a `DuplicateUsername` iff the error is a `UNIQUE_VIOLATION` and the constraint is
    /// `users_username_key`. Otherwise it returns an `UnknownError`
    ///
    /// # Parameters
    /// - `e` - The error to convert
    ///
    /// # Returns
    /// The new error code
    fn from(e: postgres::Error) -> Self {
        let mut result = None;

        if e.code() == Some(&SqlState::UNIQUE_VIOLATION) {
            let db_error: Option<DbError> = e
                .into_source()
                .and_then(|e| e.downcast_ref::<DbError>().cloned());

            result = db_error
                .and_then(|e| e.constraint().map(|c| c.to_owned()))
                .map(|constraint| match constraint.as_ref() {
                    "users_username_key" => SaveUserError::DuplicateUsername,
                    _ => {
                        tracing::warn!("Unexpected constraint violation error: {:?}", constraint);
                        SaveUserError::UnknownError
                    }
                });
        } else {
            tracing::warn!("Unexpected database error: {:?}", e);
        }

        result.unwrap_or(SaveUserError::UnknownError)
    }
}
