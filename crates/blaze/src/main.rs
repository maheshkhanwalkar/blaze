mod command;
mod diff;
mod fdisk;
mod file;
mod init;
mod kv;
mod merge;
mod vfs;

use crate::command::Command;
use crate::diff::DiffCommand;
use crate::fdisk::FdiskCommand;
use crate::init::InitCommand;
use crate::kv::KVCommand;
use crate::merge::MergeCommand;
use crate::vfs::vfs_set_cwd;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::exit;

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
    /// Initialize a new blaze repository.
    Init,
    /// Manipulate the internal key-value store.
    Kv {
        #[command(subcommand)]
        kv_sub_command: KvSubCommand,
    },
    /// Manage partitions
    Fdisk {
        #[command(subcommand)]
        fdisk_sub_command: FdiskSubCommand,
    },
}

#[derive(Debug, Subcommand)]
enum KvSubCommand {
    /// Get a value from the key-value store.
    #[command(name = "get")]
    Get {
        #[clap(flatten)]
        group: KvPartitionArgGroup,
        /// Key to get
        key: String,
    },
    /// Insert a value into the key-value store.
    #[command(name = "put")]
    Put {
        #[clap(flatten)]
        group: KvPartitionArgGroup,
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

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct KvPartitionArgGroup {
    /// Use the global key-value store if set to true
    #[clap(long, default_value_t = false)]
    global: bool,
    /// Use the specified partition if set
    #[arg(long, conflicts_with = "global")]
    partition: Option<String>,
}

#[derive(Debug, Subcommand)]
enum FdiskSubCommand {
    /// List all the local partitions in the repository
    #[command(name = "ls")]
    List,
    /// Create a new partition
    #[command(name = "create")]
    Create {
        /// Name of the partition to create
        name: String,
        /// Path to the partition. This must be a relative path from the repository root.
        path: String,
    },
}

fn main() {
    let args = Args::parse();
    if should_set_cwd(&args) {
        handle_error(&vfs_set_cwd());
    }

    let command: Box<dyn Command> = match args.action {
        Action::Diff { first, second } => Box::new(DiffCommand { first, second }),
        Action::Merge { original, v1, v2 } => Box::new(MergeCommand { original, v1, v2 }),
        Action::Init => Box::new(InitCommand),
        Action::Kv { kv_sub_command } => {
            let command = build_kv_command(kv_sub_command);
            Box::new(command)
        }
        Action::Fdisk { fdisk_sub_command } => match fdisk_sub_command {
            FdiskSubCommand::List => Box::new(FdiskCommand::List),
            FdiskSubCommand::Create { name, path } => Box::new(FdiskCommand::Create { name, path }),
        },
    };
    let result = command.execute();
    handle_error(&result);
}

fn build_kv_command(kv_sub_command: KvSubCommand) -> KVCommand {
    match kv_sub_command {
        KvSubCommand::Get { group, key } => KVCommand::Get {
            partition: partition_name(group.partition, group.global),
            key,
        },
        KvSubCommand::Put { group, key, value } => KVCommand::Put {
            partition: partition_name(group.partition, group.global),
            key,
            value,
        },
        KvSubCommand::Hash { key } => KVCommand::Hash { key },
    }
}

fn handle_error(result: &Result<()>) {
    if let Err(e) = result {
        eprintln!("error. {}", e);
        exit(1);
    }
}

fn partition_name(partition: Option<String>, global: bool) -> String {
    if global {
        String::from("global")
    } else {
        partition.unwrap()
    }
}

fn should_set_cwd(args: &Args) -> bool {
    /*
     * Don't set the cwd if the user is initializing a new repository.
     * That's because there's no repository root yet, so we don't know where to set the cwd.
     */
    match args.action {
        Action::Init => false,
        _ => true,
    }
}
