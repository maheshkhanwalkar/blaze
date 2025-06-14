mod command;
mod diff;
mod file;
mod init;
mod key;
mod kv;
mod merge;
mod vfs;

use crate::command::Command;
use crate::diff::runner::DiffCommand;
use crate::init::InitCommand;
use crate::kv::KVCommand;
use crate::merge::runner::MergeCommand;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Display the diff between two files.
    Diff { first: String, second: String },
    /// 3-way merge between original (base), v1 and v2.
    Merge {
        original: String,
        v1: String,
        v2: String,
    },
    /// Initialise a new blaze repository.
    Init,
    Kv {
        #[command(subcommand)]
        kv_sub_command: KvSubCommand,
    },
}

#[derive(Debug, Subcommand)]
enum KvSubCommand {
    /// Get a value from the key-value store.
    #[command(name = "get")]
    Get {
        /// Use the global key-value store if set to true
        #[clap(long, default_value_t = false)]
        global: bool,
        /// Use the specified partition if set
        #[arg(long, conflicts_with = "global")]
        partition: Option<String>,
        /// Key to get
        key: String,
    },
    /// Insert a value into the key-value store.
    #[command(name = "put")]
    Put {
        /// Use the global key-value store if set to true
        #[clap(long, default_value_t = false)]
        global: bool,
        /// Use the specified partition if set
        #[arg(long, conflicts_with = "global")]
        partition: Option<String>,
        /// Key to insert into the store
        key: String,
        /// Associated value for the specified key
        value: String,
    },
    /// View the hash of a key.
    #[command(name = "hash")]
    Hash {
        /// Key to hash
        key: String,
    },
}

fn main() {
    let args = Args::parse();
    let command: Box<dyn Command> = match args.action {
        Action::Diff { first, second } => Box::new(DiffCommand { first, second }),
        Action::Merge { original, v1, v2 } => Box::new(MergeCommand { original, v1, v2 }),
        Action::Init => Box::new(InitCommand),
        Action::Kv { kv_sub_command } => {
            let command = match kv_sub_command {
                KvSubCommand::Get {
                    global,
                    partition,
                    key,
                } => KVCommand::Get {
                    partition: partition_name(partition, global),
                    key,
                },
                KvSubCommand::Put {
                    global,
                    partition,
                    key,
                    value,
                } => KVCommand::Put {
                    partition: partition_name(partition, global),
                    key,
                    value,
                },
                KvSubCommand::Hash { key } => KVCommand::Hash { key },
            };
            Box::new(command)
        }
    };
    let result = command.execute();
    if let Err(e) = result {
        eprintln!("error. {}", e);
    }
}

fn partition_name(partition: Option<String>, global: bool) -> String {
    if global {
        String::from("global")
    } else {
        partition.unwrap()
    }
}
