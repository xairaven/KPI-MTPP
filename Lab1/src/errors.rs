#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid task number - {0}. Valid task numbers are 1-5.")]
    InvalidTaskNumber(u8),

    #[error("Setup failed. {0}")]
    SetupError(#[from] SetupError),

    #[error("Teardown failed. {0}")]
    TeardownError(#[from] TeardownError),
}

#[derive(Debug, thiserror::Error)]
pub enum SetupError {}

#[derive(Debug, thiserror::Error)]
pub enum TeardownError {}
