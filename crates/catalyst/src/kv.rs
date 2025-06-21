use crate::key::construct_key;
use crate::kv::ItemType::{Key, Value};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{create_dir, read_dir, read_to_string, write, ReadDir};

/// `KeyValueStore` implements a key-value store which uses the local
/// filesystem for persistence. One important aspect of the key-value store
/// is that it does not support deletion -- as it is not required for version
/// control and could lead to repository corruption.
///
/// # Variants
///
/// - `Global`:
///   Represents a global key-value store that is not partition-specific. This
///   is generally used to store repository-wide data.
///
/// - `Partition(&'a String)`:
///   Represents a partitioned key-value store with a reference to a `String`
///   that specifies the name of the partition. Any data local to a partition
///   is stored within its associated k-v store.
/// ```
pub enum KeyValueStore<'a> {
    Global,
    Partition(&'a String),
}

/// The `KeyMetadata` struct is used to associate a key with optional metadata.
///
/// # Fields
///
/// * `key` (`String`):
///   A required field representing the key as a string.
///
/// * `metadata` (`Option<String>`):
///   An optional field to store additional information or metadata related to the key. If no metadata
///   is provided, this will be `None`.
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub key: String,
    pub metadata: Option<String>,
}

/// KeyCollection represents a collection of keys in the key-value store.
/// It implements the `Iterator` trait, allowing it to be used in a for loop.
pub struct KeyCollection {
    db_itr: ReadDir,
    parent_path: String,
}

impl Iterator for KeyCollection {
    type Item = KeyMetadata;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(Ok(entry)) = self.db_itr.next() else {
            return None;
        };

        let key_dir = entry
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // TODO need to see if there's an elegant way to handle the error -- right now
        //  it'll just panic if the file is not found or can't be deserialized.
        let key_md_path = format!("{}/{key_dir}/key", self.parent_path);
        let k_md: KeyMetadata =
            serde_json::from_str(read_to_string(key_md_path).unwrap().as_str()).unwrap();
        Some(k_md)
    }
}

#[derive(Debug)]
pub struct KeyValueStoreError {
    msg: &'static str,
}

impl KeyValueStoreError {
    pub fn new(msg: &'static str) -> Self {
        Self { msg }
    }
}

impl Display for KeyValueStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for KeyValueStoreError {}

enum ItemType {
    Key,
    Value,
}

const KV_PREFIX: &str = ".blaze/db";

impl KeyValueStore<'_> {
    /// Retrieves a value associated with the given key.
    ///
    /// This method looks up the provided `key` in the underlying storage.
    /// If the file cannot be read (e.g., if it does not exist or there is an error during reading),
    /// it returns `None`. If the value is successfully retrieved, it wraps the value in `Some`.
    ///
    /// # Parameters
    /// - `key`: A string slice representing the key to retrieve the associated value.
    ///
    /// # Returns
    /// `Option<String>`:
    /// - `Some(String)`: The value associated with the key, if it exists and is successfully read.
    /// - `None`: If no value is found or there is an error reading the value.
    ///
    /// # Errors
    /// This function does not explicitly return errors but propagates them internally,
    /// defaulting to `None` for any failure.
    /// ```
    pub fn get(&self, key: &str) -> Option<String> {
        let hash_key = construct_key(key);
        let kv_path = self.get_kv_path(&hash_key, Value);

        read_to_string(kv_path).ok()
    }

    /// Retrieves the metadata associated with a specific key.
    ///
    /// This function takes a string `key` as input and attempts to read the associated metadata
    /// from storage. If the metadata is found and can be successfully deserialized into the
    /// `KeyMetadata` type, it is returned. Otherwise, the function returns `None`.
    ///
    /// # Parameters
    /// - `key`: A string slice representing the key whose metadata needs to be retrieved.
    ///
    /// # Returns
    /// - `Option<KeyMetadata>`: Returns `Some(KeyMetadata)` if the metadata is successfully
    ///   retrieved and deserialized; otherwise, returns `None`.
    pub fn get_metadata(&self, key: &str) -> Option<KeyMetadata> {
        let hash_key = construct_key(key);
        let Ok(value) = read_to_string(self.get_kv_path(&hash_key, Key)) else {
            return None;
        };
        serde_json::from_str::<KeyMetadata>(&value).ok()
    }

    /// Retrieves the collection of keys stored in the key value store.
    ///
    /// This function returns an iterable `KeyCollection` object. Iterating on this object will
    /// return a `KeyMetadata` object, which contains both the key and associated metadata.
    ///
    /// # Returns
    /// - `Ok(KeyCollection)` containing an iterable collection for the keys.
    /// - `Err(KeyValueStoreError)` if there is an error reading the key-value store.
    ///
    /// # Errors
    /// - Returns `KeyValueStoreError` with a descriptive message if the function fails to read
    /// the key-value store.
    ///
    /// # Example
    /// ```rust
    /// use catalyst::kv::KeyValueStore;
    ///
    /// let kv_store = KeyValueStore::Global;
    /// let keys = kv_store.get_keys()?;
    ///
    /// for key in keys {
    ///     // Process each key
    /// }
    /// ```
    pub fn get_keys(&self) -> Result<KeyCollection, KeyValueStoreError> {
        let parent_path = self.get_db_root();
        let db_itr =
            read_dir(&parent_path).map_err(|_| KeyValueStoreError::new("failed to read db"))?;
        Ok(KeyCollection {
            db_itr,
            parent_path,
        })
    }

    /// Inserts a key-value pair into the key-value store with optional metadata.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key for the key-value pair.
    /// * `metadata` - An optional string that contains metadata associated with the key.
    /// * `value` - A string slice that holds the value corresponding to the key.
    ///
    /// # Returns
    ///
    /// Returns a `Result`:
    /// - `Ok(())` if the key-value pair is successfully inserted into the key-value store.
    /// - `Err(KeyValueStoreError)` if an error occurs during the process.
    ///
    /// # Example
    ///
    /// ```rust
    /// use catalyst::kv::KeyValueStore;
    ///
    /// let mut store = KeyValueStore::Global; // Assume KeyValueStore is already defined.
    /// let result = store.put("exampleKey", Some("exampleMetadata".to_string()), "exampleValue");
    /// assert!(result.is_ok());
    /// ```
    pub fn put(
        &mut self,
        key: &str,
        metadata: Option<String>,
        value: &str,
    ) -> Result<(), KeyValueStoreError> {
        let hash_key = construct_key(key);
        let k_data: KeyMetadata = KeyMetadata {
            key: String::from(key),
            metadata,
        };
        let k_serde = serde_json::to_string(&k_data)
            .or_else(|_| Err(KeyValueStoreError::new("failed to serialize key")))?;

        create_dir(self.get_key_dir(&hash_key))
            .or_else(|_| Err(KeyValueStoreError::new("failed to insert key structure")))?;
        write(self.get_kv_path(&hash_key, Key), k_serde)
            .or_else(|_| Err(KeyValueStoreError::new("failed to insert key metadata")))?;
        write(self.get_kv_path(&hash_key, Value), value)
            .or_else(|_| Err(KeyValueStoreError::new("failed to insert key")))
    }

    fn get_kv_path(&self, key: &String, item_type: ItemType) -> String {
        let item_name = match item_type {
            Key => "key",
            Value => "value",
        };

        let db_root = self.get_db_root();
        format!("{}/{}/{}", db_root, key, item_name)
    }

    fn get_db_root(&self) -> String {
        match &self {
            KeyValueStore::Global => format!("{KV_PREFIX}/global"),
            KeyValueStore::Partition(name) => {
                let partition_path = KeyValueStore::Global.get(name).unwrap();
                format!("{partition_path}/{KV_PREFIX}")
            }
        }
    }

    fn get_key_dir(&self, key: &String) -> String {
        format!("{}/{}", self.get_db_root(), key)
    }
}
