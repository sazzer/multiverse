use crate::World;
use serde_json::Value;

impl World {
    /// Extract the first value from the last HTTP Response that matches the provided JSON Path
    ///
    /// # Parameters
    /// - `selector_path` - The JSON Path to match
    ///
    /// # Returns
    /// The first value that matches the path, or `None` if nothing matches
    pub fn extract_response_value<S>(&self, selector_path: S) -> Option<Value>
    where
        S: Into<String>,
    {
        let response = self.last_response();
        assert!(response.is_some());

        let response = response.unwrap();
        let payload = response.to_json().unwrap();

        let selector = jsonpath::Selector::new(&selector_path.into()).unwrap();
        let mut selected = selector.find(&payload);

        selected.find(|_| true).cloned()
    }
}
