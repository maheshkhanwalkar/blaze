use crate::key::construct_key;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::{read_to_string, write};

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

const KV_PREFIX: &str = ".blaze/db";
const KV_PARTITIONS_DIR: &str = ".blaze/db/partitions";

impl KeyValueStore<'_> {
    pub fn get(&self, key: &str) -> Option<String> {
        let hash_key = construct_key(key);
        let Ok(value) = read_to_string(self.get_kv_path(&hash_key)) else {
            return None;
        };
        Some(value)
    }

    pub fn put(&mut self, key: &str, value: &str) -> Result<(), KeyValueStoreError> {
        let hash_key = construct_key(key);
        write(self.get_kv_path(&hash_key), value)
            .or_else(|_| Err(KeyValueStoreError::new("failed to insert key")))
    }

    pub fn create_partition_path(&mut self, name: &str) -> Result<(), KeyValueStoreError> {
        match &self {
            KeyValueStore::Partition(_) => {
                return Err(KeyValueStoreError::new(
                    "cannot create a partition on a partitioned key-value store",
                ));
            }
            _ => {}
        }

        if self.get(name).is_some() {
            return Err(KeyValueStoreError::new("partition already exists"));
        }

        let partition_path = format!("{KV_PARTITIONS_DIR}/{}", name);
        fs::create_dir(partition_path)
            .or_else(|_| Err(KeyValueStoreError::new("failed to create partition path")))
    }

    pub fn get_partitions(&self) -> Result<Vec<String>, KeyValueStoreError> {
        let mut partitions = vec![];
        let Ok(dir_iter) = fs::read_dir(KV_PARTITIONS_DIR) else {
            return Err(KeyValueStoreError::new(
                "failed to read partition directory",
            ));
        };

        for entry in dir_iter {
            let Ok(entry) = entry else {
                continue;
            };

            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap().to_str().unwrap();
                partitions.push(name.to_string());
            }
        }
        Ok(partitions)
    }

    fn get_kv_path(&self, key: &String) -> String {
        match &self {
            KeyValueStore::Global => format!("{KV_PREFIX}/global/{}", key),
            KeyValueStore::Partition(name) => format!("{KV_PREFIX}/partitions/{}/{}", name, key),
        }
    }
}
