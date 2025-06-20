use crate::command::{Command, CommandExecutionError};
use crate::file::{find_dir, is_dir_empty};
use catalyst::kv::KeyValueStore;

pub enum FdiskCommand {
    List,
    Create { name: String, path: String },
}

impl Command for FdiskCommand {
    fn execute(&self) -> Result<(), CommandExecutionError> {
        match &self {
            FdiskCommand::List => {
                let kv_store = KeyValueStore::Global;
                let Ok(partitions) = kv_store.get_partitions() else {
                    return Err(CommandExecutionError {
                        message: String::from("could not get partitions"),
                    });
                };

                partitions.iter().for_each(|partition| {
                    let path = kv_store.get(partition).unwrap();
                    println!("{} => {}", partition, path);
                });
                Ok(())
            }
            FdiskCommand::Create { name, path } => {
                let mut kv_store = KeyValueStore::Global;
                validate_partition_path(path)?;

                if let Err(msg) = kv_store.create_partition_path(name) {
                    return Err(CommandExecutionError {
                        message: String::from(msg),
                    });
                }

                kv_store.put(name, path).or_else(|e| {
                    Err(CommandExecutionError {
                        message: String::from(e),
                    })
                })
            }
        }
    }
}

fn validate_partition_path(path: &str) -> Result<(), CommandExecutionError> {
    if find_dir(path, false).is_none() {
        return Err(CommandExecutionError {
            message: format!("directory {} does not exist", path),
        });
    }

    /*
     * This is a temporary limitation to make partition creation easy; however,
     * long term, we want to support creating partitions in non-empty directories and
     * handle the history correctly.
     */
    if let Ok(true) = is_dir_empty(path) {
        Ok(())
    } else {
        Err(CommandExecutionError {
            message: format!("directory {} is not empty", path),
        })
    }
}
