use crate::command::{Command, CommandExecutionError};
use catalyst::key::construct_key;
use catalyst::kv::KeyValueStore;

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
