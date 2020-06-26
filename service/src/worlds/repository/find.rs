use super::WorldRepository;
use crate::{
    model::{Page, Pagination, SortDirection, SortFields},
    worlds::{service::WorldsFilters, WorldID, WorldModel, WorldSortField},
};

impl WorldRepository {
    /// Find a world in the database
    ///
    /// # Parameters
    /// - `id` - The ID of the world to find
    ///
    /// # Returns
    /// The world world, if it exists
    #[tracing::instrument(skip(self))]
    pub fn find_world_by_id(&self, id: WorldID) -> Option<WorldModel> {
        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");
        connection
            .query_opt("SELECT * FROM worlds WHERE world_id = $1", &[&id])
            .expect("Failed to query for world by ID")
            .map(|row| self.parse_row(&row))
    }

    pub fn search_worlds(
        &self,
        filters: &WorldsFilters,
        sorts: &SortFields<WorldSortField>,
        pagination: &Pagination,
    ) -> Page<WorldModel> {
        // Collect together the Where clauses and Binds
        let mut where_clauses: Vec<String> = vec![];
        let mut binds: Vec<&(dyn postgres::types::ToSql + Sync)> = vec![];

        if let Some(owner) = &filters.owner {
            binds.push(owner);
            where_clauses.push(format!("owner_id = ${}", binds.len()));
        }
        let where_clause = if where_clauses.is_empty() {
            "".to_owned()
        } else {
            format!("WHERE {}", where_clauses.join(" AND "))
        };

        // Collect together the Sort clauses
        let mut sort_clauses: Vec<String> = sorts
            .iter()
            .map(|sort| {
                let sort_field = match &sort.field {
                    WorldSortField::Created => "worlds.created",
                    WorldSortField::Updated => "worlds.updated",
                    WorldSortField::Name => "worlds.name",
                    WorldSortField::Owner => "users.display_name",
                };
                let sort_direction = match (&sort.field, &sort.direction) {
                    (_, SortDirection::Ascending) => "ASC",
                    (_, SortDirection::Descending) => "DESC",
                    (WorldSortField::Created, SortDirection::Default) => "DESC",
                    (WorldSortField::Updated, SortDirection::Default) => "DESC",
                    (_, SortDirection::Default) => "ASC",
                };

                format!("{} {}", sort_field, sort_direction)
            })
            .collect();
        sort_clauses.push("worlds.updated DESC".to_owned());
        let sort_clause = format!("ORDER BY {}", sort_clauses.join(", "));

        let mut connection = self
            .database
            .checkout()
            .expect("Failed to get database connection");

        // Build and run the actual SELECT to get the matching records
        let select_query = format!(
            "SELECT worlds.* FROM worlds JOIN users ON worlds.owner_id = users.user_id {} {} OFFSET {} LIMIT {}",
            where_clause, sort_clause, pagination.offset, pagination.count
        );

        let records: Vec<WorldModel> = connection
            .query(select_query.as_str(), &binds[..])
            .map(|rows| rows.iter().map(|row| self.parse_row(row)).collect())
            .expect("Failed to select worlds matching query");

        let total = if records.is_empty()
            || pagination.offset + records.len() as u64 == pagination.count
        {
            // We can't correctly calculate the total number of records so query for them
            let count_query = format!("SELECT COUNT(*)::INTEGER AS c FROM worlds {}", where_clause);
            let count: i32 = connection
                .query_one(count_query.as_str(), &binds[..])
                .map(|row| row.get("c"))
                .expect("Failed to count worlds matching query");
            tracing::debug!(count = ?count, "Count of matching worlds from database");
            count as u64
        } else {
            pagination.offset + pagination.count
        };

        Page {
            entries: records,
            offset: pagination.offset,
            total,
        }
    }
}
