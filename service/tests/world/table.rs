use cucumber_rust::Step;
use std::collections::HashMap;

/// Representation of a data table where each column is a set of data, with the first column being the labels
pub struct VerticalTable<'a> {
    /// The actual contents of the table
    contents: HashMap<&'a String, &'a String>,
}

impl<'a> From<&'a Step> for VerticalTable<'a> {
    /// Convert the data table attached to a Cucumber step into a `VerticalTable`.
    /// This treats the first column as the keys and the second column as the values
    ///
    /// # Parameters
    /// - `step` - The step to get the data from
    ///
    /// # Returns
    /// The `VerticalTable` that represents this step
    fn from(step: &'a Step) -> Self {
        let mut contents = HashMap::new();

        if let Some(table) = &step.table {
            tracing::debug!(rows = ?table.rows, "Parsing table rows");
            for row in &table.rows {
                if let (Some(key), Some(value)) = (row.get(0), row.get(1)) {
                    contents.insert(key, value);
                } else {
                    tracing::warn!(row = ?row, "Table row has less than 2 columns");
                }
            }
        } else {
            tracing::warn!("No table attached to step");
        }

        Self { contents }
    }
}

impl<'a> VerticalTable<'a> {
    /// See if the given key is present in the table, and if so and it has a value then call the callback
    /// with this value.
    ///
    /// # Parameters
    /// - `key` - The key to look up
    /// - `callback` - The callback to trigger if the value is present
    pub fn if_present<S, F>(&self, key: S, callback: F)
    where
        S: Into<String>,
        F: FnOnce(&'a String),
    {
        let value = self.contents.get(&key.into());
        if let Some(value) = value {
            callback(*value);
        }
    }
}
