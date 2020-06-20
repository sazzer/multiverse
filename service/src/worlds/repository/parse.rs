use super::WorldRepository;
use crate::{model::Identity, worlds::model::*};
use postgres::row::Row;

impl WorldRepository {
    /// Parse a row from the database into a World Model
    ///
    /// # Parameters
    /// - `row` - The row to parse
    ///
    /// # Returns
    /// The world that the row represented
    pub(super) fn parse_row(&self, row: &Row) -> WorldModel {
        WorldModel {
            identity: Identity {
                id: row.get("world_id"),
                version: row.get("version"),
                created: row.get("created"),
                updated: row.get("updated"),
            },
            data: WorldData {
                owner: row.get("owner_id"),
                name: row.get("name"),
                description: row.get("description"),
                url_slug: row.get("url_slug"),
            },
        }
    }
}
