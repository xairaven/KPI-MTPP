use crate::errors::Error;
use crate::task;
use crate::task::benchmark::{BenchmarkMetadata, Benchmarkable};
use crate::task::executor::Executable;
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;

pub struct MonteCarlo;

impl Reportable for MonteCarlo {
    fn name(&self) -> &'static str {
        "CPU-Bound: Monte-Carlo"
    }
}

impl Benchmarkable for MonteCarlo {
    fn benchmark_tasks(&self) -> Vec<BenchmarkMetadata> {
        task::BENCHMARKS.to_vec()
    }
}

impl Measurable for MonteCarlo {}
impl Manageable for MonteCarlo {}

impl Executable for MonteCarlo {
    fn run_sequential(&self) -> Result<(), Error> {
        todo!()
    }

    fn run_threads(&self, threads: usize) -> Result<(), Error> {
        todo!()
    }

    fn run_workers(&self, workers: usize) -> Result<(), Error> {
        todo!()
    }

    fn run_process_chunk(
        &self, process_index: usize, total_processes: usize,
    ) -> Result<(), Error> {
        todo!()
    }
}
