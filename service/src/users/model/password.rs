use argonautica::{Hasher, Verifier};
use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

/// Typesafe representation of the hashed password of some user
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromSql)]
pub struct Password(String);

impl Password {
    /// Take a plaintext password and hash it, then return a Password object wrapping this hashed password
    ///
    /// # Parameters
    /// - `plaintext` - The plaintext password to hash
    ///
    /// # Returns
    /// The hashed password
    pub fn from_plaintext<S>(plaintext: S) -> Password
    where
        S: Into<String>,
    {
        let hash = Hasher::default()
            .with_password(plaintext.into())
            .opt_out_of_secret_key(true)
            .hash()
            .unwrap();

        Self(hash)
    }
}

impl PartialEq<String> for Password {
    fn eq(&self, other: &String) -> bool {
        Verifier::default()
            .with_hash(self.0.clone())
            .with_password(other.clone())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = Password::from_plaintext("secret");
        println!("{:?}", password);
    }

    #[test]
    fn test_rehash_password() {
        let password = Password::from_plaintext("secret");
        let password2 = Password::from_plaintext("secret");

        assert_ne!(password, password2);
    }

    #[test]
    fn test_verify_password() {
        let password = Password::from_plaintext("secret");
        assert_eq!(password, "secret".to_owned());
        assert_ne!(password, "Secret".to_owned());
        assert_ne!(password, "secrets".to_owned());
    }
}
