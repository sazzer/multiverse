use super::UserRepository;
use crate::model::Identity;
use crate::users::model::*;
use tokio_postgres::row::Row;

impl UserRepository {
    /// Parse a row from the database into a User Model
    ///
    /// # Parameters
    /// - `row` - The row to parse
    ///
    /// # Returns
    /// The user that the row represented
    pub(super) fn parse_row(&self, row: &Row) -> UserModel {
        UserModel {
            identity: Identity {
                id: row.get("user_id"),
                version: row.get("version"),
                created: row.get("created"),
                updated: row.get("updated"),
            },
            data: UserData {
                username: row.get("username"),
                display_name: row.get("display_name"),
                email_address: row.get("email_address"),
                avatar_url: row.get("avatar_url"),
                password: row.get("password"),
            },
        }
    }
}
