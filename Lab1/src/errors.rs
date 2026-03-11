use crate::logs::LogError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User input. {0}")]
    Cli(#[from] CliError),

    #[error("Setup failed. {0}")]
    Setup(#[from] SetupError),

    #[error("Teardown failed. {0}")]
    Teardown(#[from] TeardownError),

    #[error("System. {0}")]
    System(#[from] SystemError),

    #[error("Logging. {0}")]
    Log(#[from] LogError),
}

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Invalid task number - {0}. Valid task numbers are 1-5.")]
    InvalidTaskNumber(usize),

    #[error("Process index is required when processes mode is enabled.")]
    MissingProcessIndex,

    #[error("Total processes is required when processes mode is enabled.")]
    MissingTotalProcesses,
}

#[derive(Debug, thiserror::Error)]
pub enum SetupError {}

#[derive(Debug, thiserror::Error)]
pub enum TeardownError {}

#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error("Failed to get current executable path. {0}")]
    CurrentExe(std::io::Error),

    #[error("Failed to spawn child process. {0}")]
    ChildProcess(std::io::Error),

    #[error("Failed to parse child process output - \"{0}\"")]
    FailedParsingChildOutput(String),

    #[error("Failed to build a rayon worker pool. {0}")]
    RayonPoolBuild(#[from] rayon::ThreadPoolBuildError),

    #[error("Thread panicked during execution.")]
    ThreadPanicked,

    #[error("Failed while waiting for child process to finish. {0}")]
    WaitingForChildProcess(std::io::Error),
}
