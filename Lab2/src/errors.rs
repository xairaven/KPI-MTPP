use crate::cli::CliError;
use crate::logs::LogError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("User input. {0}")]
    Cli(#[from] CliError),

    #[error("Logger. {0}")]
    Log(#[from] LogError),

    #[error("System error: {0}")]
    System(#[from] SystemError),
}

#[derive(Debug, Error)]
pub enum SystemError {
    #[error("Failed to get current executable path. {0}")]
    CurrentExe(io::Error),
}
