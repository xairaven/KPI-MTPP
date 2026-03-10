use crate::errors::{SetupError, TeardownError};

pub trait Manageable {
    // Prepares necessary data, files, or environment before the execution
    fn setup(&mut self) -> Result<(), SetupError> {
        Ok(())
    }

    // Cleans up resources, deletes files, or frees memory after the execution
    fn teardown(&mut self) -> Result<(), TeardownError> {
        Ok(())
    }
}
