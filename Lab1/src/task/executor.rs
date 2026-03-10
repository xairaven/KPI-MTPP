use crate::errors::Error;

pub trait Executable {
    // Executes the task sequentially in a single main thread
    fn run_sequential(&self) -> Result<(), Error>;

    // Executes the task using std::thread
    fn run_threads(&self, threads: usize) -> Result<(), Error>;

    // Executes the task using a thread pool with work-stealing mechanism (`rayon` crate)
    fn run_workers(&self, workers: usize) -> Result<(), Error>;

    // Executes the task by spawning isolated processes
    fn run_processes(&self, processes: usize) -> Result<(), Error>;
}
