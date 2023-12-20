/*!
The KvStore store key/value pairs.
 */
#![deny(missing_docs)]

use std::collections::HashMap;


/// A simple key-value store implementation.
///
/// This key-value store allows you to store and retrieve string values using string keys.
/// It supports basic operations such as setting a key-value pair, getting the value of a key,
/// and removing a key from the store.
///
/// # Examples
///
/// ```
/// use blade_turbo::KvStore;
///
/// let mut store = KvStore::new();
///
/// store.set("name".to_string(), "John".to_string());
/// assert_eq!(store.get("name".to_string()), Some("John".to_string()));
///
/// store.remove("name".to_string());
/// assert_eq!(store.get("name".to_string()), None);
/// ```
pub struct KvStore {
    data: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Generate a new `KvStore` instance.
    ///
    /// # Returns
    ///
    /// A new `KvStore` instance.
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }

    /// Set the value of a string key to a string.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set.
    /// * `value` - The value to set.
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Get the string value of a given string key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve the value for.
    ///
    /// # Returns
    ///
    /// The value associated with the given key, if it exists.
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    /// Remove a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove.
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut store = KvStore::new();
        store.set("1".to_owned(), "value1".to_owned());
        store.set("2".to_owned(), "value2".to_owned());

        assert_eq!(store.get("1".to_owned()), Some("value1".to_owned()));
        assert_eq!(store.get("2".to_owned()), Some("value2".to_owned()));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let store = KvStore::new();

        assert_eq!(store.get("nonexistent".to_owned()), None);
    }

    #[test]
    fn test_remove() {
        let mut store = KvStore::new();
        store.set("1".to_owned(), "value1".to_owned());
        store.set("2".to_owned(), "value2".to_owned());

        store.remove("1".to_owned());

        assert_eq!(store.get("1".to_owned()), None);
        assert_eq!(store.get("2".to_owned()), Some("value2".to_owned()));
    }
}
