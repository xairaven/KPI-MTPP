use crate::errors::{Error, SystemError};
use std::process::{Command, Stdio};

pub trait Executable {
    // Executes the task sequentially in a single main thread
    fn run_sequential(&self) -> Result<(), Error>;

    // Executes the task using std::thread
    fn run_threads(&self, threads: usize) -> Result<(), Error>;

    // Executes the task using a thread pool with work-stealing mechanism (`rayon` crate)
    fn run_workers(&self, workers: usize) -> Result<(), Error>;

    // Executes the task by spawning isolated processes
    fn run_processes(&self, task_index: usize, processes: usize) -> Result<(), Error> {
        // Get the absolute path to the currently running executable
        let exe_path = std::env::current_exe().map_err(SystemError::CurrentExe)?;

        let mut children = Vec::new();

        // Spawn the requested number of child processes
        for i in 0..processes {
            let mut cmd = Command::new(&exe_path);

            // Pass the necessary arguments for the child to identify its chunk
            cmd.arg("--task").arg(task_index.to_string())
                .arg("--processes-mode")
                .arg("--process-index").arg(i.to_string())
                .arg("--total-processes").arg(processes.to_string())
                // Suppress child output so it doesn't mess up our benchmark report console
                .stdout(Stdio::null());

            let child = cmd.spawn().map_err(SystemError::ChildProcess)?;
            children.push(child);
        }

        // Wait for all child processes to finish their specific chunks
        for mut child in children {
            child.wait().map_err(SystemError::WaitingForChildProcess)?;
        }

        Ok(())
    }

    // Executes a specific chunk of the task (Child execution)
    fn run_process_chunk(
        &self, process_index: usize, total_processes: usize,
    ) -> Result<(), Error>;
}
