use crate::command::Command;
use crate::file::{find_dir, is_dir_empty};
use anyhow::Result;
use catalyst::kv::KeyValueStore;

pub enum FdiskCommand {
    List,
    Create { name: String, path: String },
}

impl Command for FdiskCommand {
    fn execute(&self) -> Result<()> {
        match &self {
            FdiskCommand::List => {
                let kv_store = KeyValueStore::Global;
                let partitions = kv_store.get_partitions()?;

                partitions.iter().for_each(|partition| {
                    let path = kv_store.get(partition).unwrap();
                    println!("{} => {}", partition, path);
                });
                Ok(())
            }
            FdiskCommand::Create { name, path } => {
                let mut kv_store = KeyValueStore::Global;

                validate_partition_path(path)?;
                kv_store.create_partition_path(name)?;
                kv_store.put(name, path)?;
                Ok(())
            }
        }
    }
}

fn validate_partition_path(path: &str) -> Result<()> {
    if find_dir(path, false).is_none() {
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
