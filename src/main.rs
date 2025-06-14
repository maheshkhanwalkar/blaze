use blaze::command::Command;
use blaze::diff::runner::DiffCommand;
use blaze::init::InitCommand;
use blaze::merge::runner::MergeCommand;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Display the diff between two files.
    Diff {
        first: String,
        second: String,
    },
    /// 3-way merge between original (base), v1 and v2.
    Merge {
        original: String,
        v1: String,
        v2: String,
    },
    /// Initialise a new blaze repository.
    Init,
}

fn main() {
    let args = Args::parse();
    let result = match args.action {
        Action::Diff { first, second } => DiffCommand { first, second }.execute(),
        Action::Merge { original, v1, v2 } => MergeCommand { original, v1, v2 }.execute(),
        Action::Init => InitCommand.execute(),
    };
    if let Err(e) = result {
        eprintln!("error. {}", e);
    }
}
