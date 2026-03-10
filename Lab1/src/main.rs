use crate::cli::InputArgs;
use clap::Parser;

fn main() {
    let args = InputArgs::parse();
}

mod benchmark;
mod cli;
mod lifecycle;
mod tasks;
