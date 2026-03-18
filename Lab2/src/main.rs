use crate::cli::{CliError, InputArgs};
use crate::errors::Error;
use crate::logs::Logger;
use crate::tasks::tasks;
use clap::Parser;

fn main() {
    let args = InputArgs::parse();

    Logger::from_args(&args).setup().unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    });

    match task_result(args) {
        Ok(report) => println!("{}", report),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        },
    }
}

fn task_result(args: InputArgs) -> Result<String, Error> {
    let task_index = args.task;
    let tasks = tasks();

    if !(1..=tasks.len()).contains(&task_index) {
        Err(CliError::UnknownTask(task_index))?;
    }

    let mut task = tasks
        .into_iter()
        .nth(task_index - 1)
        .ok_or(CliError::UnknownTask(task_index))?;

    let report = task.report()?;

    Ok(report.get_text())
}

mod cli;
mod errors;
mod logs;
mod tasks;

mod task {
    pub mod benchmark;
    pub mod executable;
    pub mod lifecycle;
    pub mod measure;
    pub mod report;
}
