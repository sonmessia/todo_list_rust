mod commands;
mod models;
mod storage;

use structopt::StructOpt;
use commands::Command;

fn main() {
    let command = Command::from_args();
    command.execute().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}
