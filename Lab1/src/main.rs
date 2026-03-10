use crate::cli::InputArgs;
use crate::errors::Error;
use crate::task::tasks;
use clap::Parser;

fn main() {
    let args = InputArgs::parse();

    if let Err(e) = run_task(args.task) {
        eprintln!("Error: {}", e);
    }
}

fn run_task(task: u8) -> Result<String, Error> {
    let tasks = tasks();
    let mut task = tasks
        .into_iter()
        .nth(task as usize)
        .ok_or(Error::InvalidTaskNumber(task))?;

    let report = task.report();

    Ok(report.text())
}

mod cli;
mod errors;
mod task;

mod tasks {
    pub mod monte_carlo;
}
