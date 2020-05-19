use argonautica::{Hasher, Verifier};
use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

/// Typesafe representation of the hashed password of some user
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromSql)]
pub struct Password(String);

/// Typesafe representation of a plaintext password
#[derive(PartialEq, Deserialize, Clone)]
pub struct Plaintext(String);

/// Errors that can occur when hashing a password
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PasswordHashError {
    /// The email address was blank
    #[error("The password was blank")]
    Blank,

    #[error("The password hashing failed")]
    HashError,
}

impl Password {
    /// Take a plaintext password and hash it, then return a Password object wrapping this hashed password
    ///
    /// # Parameters
    /// - `plaintext` - The plaintext password to hash
    ///
    /// # Returns
    /// The hashed password
    pub fn from_plaintext(plaintext: Plaintext) -> Result<Password, PasswordHashError> {
        if plaintext.0.trim().is_empty() {
            Err(PasswordHashError::Blank)?
        }

        let hash = Hasher::default()
            .with_password(plaintext.0)
            .opt_out_of_secret_key(true)
            .hash()
            .map_err(|e| {
                tracing::warn!("Failed to hash password: {:?}", e);
                PasswordHashError::HashError
            })?;

        Ok(Self(hash))
    }
}

impl PartialEq<Plaintext> for Password {
    fn eq(&self, other: &Plaintext) -> bool {
        Verifier::default()
            .with_hash(self.0.clone())
            .with_password(other.0.clone())
            .verify()
            .unwrap()
    }
}

impl ToSql for Password {
    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }

    accepts!(TEXT, VARCHAR);
    to_sql_checked!();
}

impl std::fmt::Debug for Plaintext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plaintext(Redacted)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = Password::from_plaintext(Plaintext("secret".to_owned())).unwrap();
        println!("{:?}", password);
    }

    #[test]
    fn test_hash_blank_password() {
        assert_eq!(
            PasswordHashError::Blank,
            Password::from_plaintext(Plaintext("".to_owned())).unwrap_err()
        );
        assert_eq!(
            PasswordHashError::Blank,
            Password::from_plaintext(Plaintext(" ".to_owned())).unwrap_err()
        );
    }

    #[test]
    fn test_rehash_password() {
        let password = Password::from_plaintext(Plaintext("secret".to_owned())).unwrap();
        let password2 = Password::from_plaintext(Plaintext("secret".to_owned())).unwrap();

        assert_ne!(password, password2);
    }

    #[test]
    fn test_verify_password() {
        let password = Password::from_plaintext(Plaintext("secret".to_owned())).unwrap();
        assert_eq!(password, Plaintext("secret".to_owned()));
        assert_ne!(password, Plaintext("Secret".to_owned()));
        assert_ne!(password, Plaintext("secrets".to_owned()));
    }
}
