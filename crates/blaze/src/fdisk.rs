use crate::command::Command;
use crate::file::{find_dir, is_dir_empty};
use crate::vfs::vfs_create_partition;
use anyhow::Result;
use catalyst::kv::KeyValueStore;
use std::env::current_dir;

pub enum FdiskCommand {
    List,
    Create { name: String, path: String },
}

impl Command for FdiskCommand {
    fn execute(&self) -> Result<()> {
        match &self {
            FdiskCommand::List => {
                let kv_store = KeyValueStore::Global;
                let partitions = kv_store.get_keys()?;

                partitions
                    .filter(|md| {
                        md.metadata.is_some() && md.metadata.as_ref().unwrap() == "partition"
                    })
                    .map(|md| md.key)
                    .for_each(|partition| {
                        let path = kv_store.get(partition.as_str()).unwrap();
                        println!("{} => {}", partition, path);
                    });
                Ok(())
            }
            FdiskCommand::Create { name, path } => {
                let mut kv_store = KeyValueStore::Global;

                validate_partition_path(path)?;
                ensure_partition_is_new(&kv_store, name)?;
                vfs_create_partition(path)?;
                kv_store.put(name, Some(String::from("partition")), path)?;
                Ok(())
            }
        }
    }
}

fn ensure_partition_is_new(kv_store: &KeyValueStore, name: &str) -> Result<()> {
    if let Some(_) = kv_store.get(name) {
        Err(anyhow::Error::msg(format!(
            "partition {} already exists",
            name
        )))
    } else {
        Ok(())
    }
}

fn validate_partition_path(path: &str) -> Result<()> {
    if find_dir(current_dir()?.as_path(), path, false).is_none() {
        return Err(anyhow::Error::msg(format!(
            "directory {} does not exist",
            path
        )));
    }

    /*
     * This is a temporary limitation to make partition creation easy; however,
     * long term, we want to support creating partitions in non-empty directories and
     * handle the history correctly.
     */
    if !is_dir_empty(path)? {
        Err(anyhow::Error::msg(format!(
            "directory {} is not empty",
            path
        )))
    } else {
        Ok(())
    }
}
