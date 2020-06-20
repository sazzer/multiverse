use postgres::error::{DbError, SqlState};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SaveWorldError {
    #[error("An unknown error occurred")]
    UnknownError,

    #[error("The URL Slug was already present for this user")]
    DuplicateUrlSlug,

    #[error("The desired owner does not exist")]
    UnknownOwner,
}

impl From<postgres::Error> for SaveWorldError {
    /// Convert a database error into a SaveWorldError.
    ///
    /// The conversion returns a `DuplicateUrlSlug` iff the error is a `UNIQUE_VIOLATION` and the constraint is
    /// `key_worlds_owner_slug`. Otherwise it returns an `UnknownError`
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
                    "key_worlds_owner_slug" => SaveWorldError::DuplicateUrlSlug,
                    _ => {
                        tracing::warn!(
                            "Unexpected unique key constraint violation error: {:?}",
                            constraint
                        );
                        SaveWorldError::UnknownError
                    }
                });
        } else if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION) {
            let db_error: Option<DbError> = e
                .into_source()
                .and_then(|e| e.downcast_ref::<DbError>().cloned());

            result = db_error
                .and_then(|e| e.constraint().map(|c| c.to_owned()))
                .map(|constraint| match constraint.as_ref() {
                    "worlds_owner_id_fkey" => SaveWorldError::UnknownOwner,
                    _ => {
                        tracing::warn!(
                            "Unexpected foreign key constraint violation error: {:?}",
                            constraint
                        );
                        SaveWorldError::UnknownError
                    }
                });
        } else {
            tracing::warn!("Unexpected database error: {:?}", e);
        }

        result.unwrap_or(SaveWorldError::UnknownError)
    }
}
