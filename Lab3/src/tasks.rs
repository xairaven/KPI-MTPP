use crate::cli::{CliError, InputArgs};
use crate::errors::Error;
use crate::task::report::Reportable;
use thiserror::Error;

pub struct BenchmarkRunner {
    tasks: Vec<Box<dyn Reportable>>,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self { tasks: vec![] }
    }
}

impl BenchmarkRunner {
    pub fn run(&mut self, args: InputArgs) -> Result<String, Error> {
        let task_index = args.task;

        if !(1..=self.tasks.len()).contains(&task_index) {
            Err(CliError::UnknownTask(task_index))?;
        }

        let task = self
            .tasks
            .get_mut(task_index - 1)
            .ok_or(CliError::UnknownTask(task_index))?;

        let report = task.report()?;

        Ok(report.get_text())
    }
}

#[derive(Debug, Error)]
pub enum TaskLogicError {
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}
