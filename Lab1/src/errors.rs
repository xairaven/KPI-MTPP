#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid task number: {0}. Valid task numbers are 1-5.")]
    InvalidTaskNumber(u8),
}
