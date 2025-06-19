use crate::command::{Command, CommandExecutionError};
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
