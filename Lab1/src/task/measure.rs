use crate::task::executor::Executable;
use crate::task::lifecycle::Manageable;
use std::time::{Duration, Instant};

pub trait Measurable: Executable + Manageable {
    // Measures the exact execution time of the sequential approach
    fn measure_sequential(&mut self) -> Duration {
        self.setup();
        let start = Instant::now();
        self.run_sequential();
        let elapsed = start.elapsed();
        self.teardown();
        elapsed
    }

    // Measures the execution time of the thread-based approach
    fn measure_threads(&mut self, num_threads: usize) -> Duration {
        self.setup();
        let start = Instant::now();
        self.run_threads(num_threads);
        let elapsed = start.elapsed();
        self.teardown();
        elapsed
    }

    // Measures the execution time of the worker-based approach
    fn measure_workers(&mut self, num_workers: usize) -> Duration {
        self.setup();
        let start = Instant::now();
        self.run_workers(num_workers);
        let elapsed = start.elapsed();
        self.teardown();
        elapsed
    }

    // Measures the execution time of the process-based approach
    fn measure_processes(&mut self, num_processes: usize) -> Duration {
        self.setup();
        let start = Instant::now();
        self.run_processes(num_processes);
        let elapsed = start.elapsed();
        self.teardown();
        elapsed
    }
}
