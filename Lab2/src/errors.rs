use crate::cli::CliError;
use crate::logs::LogError;
use crate::task::benchmark::BenchmarkError;
use crate::tasks::TaskLogicError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Benchmark. {0}")]
    Benchmark(#[from] BenchmarkError),

    #[error("User input. {0}")]
    Cli(#[from] CliError),

    #[error("Logger. {0}")]
    Log(#[from] LogError),

    #[error("System. {0}")]
    System(#[from] SystemError),

    #[error("Task Logic. {0}")]
    TaskLogic(#[from] TaskLogicError),
}

#[derive(Debug, Error)]
pub enum SystemError {
    #[error("Failed to get current executable path. {0}")]
    CurrentExe(io::Error),

    #[error("Failed to build rayon pool. {0}")]
    WorkerPoolBuild(#[from] rayon::ThreadPoolBuildError),
}
