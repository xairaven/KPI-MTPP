use crate::cli::InputArgs;
use crate::logs::Logger;
use clap::Parser;

fn main() {
    let args = InputArgs::parse();

    Logger::from_args(&args).setup().unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    });
}

mod cli;
mod errors;
mod logs;
mod task;
