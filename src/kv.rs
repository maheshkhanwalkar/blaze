use crate::command::{Command, CommandExecutionError};
use crate::key::construct_key;
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

const KV_PREFIX: &str = ".blaze/db";

impl KeyValueStore<'_> {
    pub fn get(&self, key: &str) -> Option<String> {
        let Ok(value) = read_to_string(self.get_kv_path(key)) else {
            return None;
        };
        Some(value)
    }

    pub fn put(&mut self, key: &str, value: &str) {
        write(self.get_kv_path(key), value).unwrap();
    }

    fn get_kv_path(&self, key: &str) -> String {
        match &self {
            KeyValueStore::Global => format!("{KV_PREFIX}/global/{}", key),
            KeyValueStore::Partition(name) => format!("{KV_PREFIX}/partition/{}/{}", name, key),
        }
    }
}

pub enum KVCommand {
    Get {
        partition: String,
        key: String,
    },
    Put {
        partition: String,
        key: String,
        value: String,
    },
    Hash {
        key: String,
    },
}

impl Command for KVCommand {
    fn execute(&self) -> Result<(), CommandExecutionError> {
        match &self {
            KVCommand::Get { partition, key } => {
                let kv_store = Self::get_kv_store(partition);
                let hash_key = construct_key(key);

                let Some(value) = kv_store.get(&hash_key) else {
                    println!("Key '{key}' not found");
                    return Ok(());
                };
                println!("{}", value);
                Ok(())
            }
            KVCommand::Put {
                partition,
                key,
                value,
            } => {
                let mut kv_store = Self::get_kv_store(partition);
                let hash_key = construct_key(key);
                kv_store.put(&hash_key, value);
                Ok(())
            }
            // Prints out the internal hash key used for a given key. This is useful
            // for introspection to see how blaze stores data internally.
            KVCommand::Hash { key } => {
                let hash_key = construct_key(key);
                println!("{}", hash_key);
                Ok(())
            }
        }
    }
}

impl KVCommand {
    fn get_kv_store(partition: &String) -> KeyValueStore {
        if partition == "global" {
            KeyValueStore::Global
        } else {
            KeyValueStore::Partition(partition)
        }
    }
}
