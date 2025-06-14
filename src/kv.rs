use crate::command::{Command, CommandExecutionError};
use crate::key::construct_key;
use std::fs::{read_to_string, write};

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
                    println!("Key {key} not found");
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
