use crate::lifecycle::TaskLifecycle;
use crate::tasks::TaskExecution;
use std::time::{Duration, Instant};

// Orchestrator struct to handle precise timing without boilerplate
pub struct Benchmarker;

impl Benchmarker {
    // Measures the exact execution time of the sequential approach
    pub fn measure_sequential<T: TaskLifecycle + TaskExecution>(
        task: &mut T,
    ) -> Duration {
        task.setup();
        let start = Instant::now();
        task.run_sequential();
        let elapsed = start.elapsed();
        task.teardown();
        elapsed
    }

    // Measures the execution time of the thread-based approach
    pub fn measure_threads<T: TaskLifecycle + TaskExecution>(
        task: &mut T, num_threads: usize,
    ) -> Duration {
        task.setup();
        let start = Instant::now();
        task.run_threads(num_threads);
        let elapsed = start.elapsed();
        task.teardown();
        elapsed
    }

    // Measures the execution time of the worker-based approach
    pub fn measure_workers<T: TaskLifecycle + TaskExecution>(
        task: &mut T, num_workers: usize,
    ) -> Duration {
        task.setup();
        let start = Instant::now();
        task.run_workers(num_workers);
        let elapsed = start.elapsed();
        task.teardown();
        elapsed
    }

    // Measures the execution time of the process-based approach
    pub fn measure_processes<T: TaskLifecycle + TaskExecution>(
        task: &mut T, num_processes: usize,
    ) -> Duration {
        task.setup();
        let start = Instant::now();
        task.run_processes(num_processes);
        let elapsed = start.elapsed();
        task.teardown();
        elapsed
    }
}
