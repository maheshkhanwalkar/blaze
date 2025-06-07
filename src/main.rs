use blaze::{command, diff, merge};
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: blaze <command> <args>");
        return;
    }

    let command = get_command(&args);
    let res = command.execute();

    match res {
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
        Ok(_) => {}
    }
}

fn get_command(args: &Vec<String>) -> Box<dyn command::Command> {
    let command = args[1].clone();
    let rem_args = args[2..].to_vec();

    match command.as_str() {
        "diff" => Box::new(diff::runner::DiffCommand { args: rem_args }),
        "merge" => Box::new(merge::runner::MergeCommand { args: rem_args }),
        &_ => panic!("unknown command: {command}"),
    }
}
