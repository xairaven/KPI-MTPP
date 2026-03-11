use crate::task::benchmark::BenchmarkMetadata;
use crate::task::report::Reportable;
use crate::tasks::factorization::Factorization;
use crate::tasks::monte_carlo::MonteCarlo;

pub const BENCHMARKS: [BenchmarkMetadata; 10] = [
    BenchmarkMetadata::Sequential,
    BenchmarkMetadata::Threads(2),
    BenchmarkMetadata::Threads(4),
    BenchmarkMetadata::Threads(8),
    BenchmarkMetadata::Workers(2),
    BenchmarkMetadata::Workers(4),
    BenchmarkMetadata::Workers(8),
    BenchmarkMetadata::Processes(2),
    BenchmarkMetadata::Processes(4),
    BenchmarkMetadata::Processes(8),
];

pub fn tasks() -> Vec<Box<dyn Reportable>> {
    vec![
        Box::new(MonteCarlo::with_iterations(100_000_000)),
        Box::new(Factorization::default()),
    ]
}

pub mod benchmark;
pub mod executor;
pub mod lifecycle;
pub mod measure;
pub mod report;
