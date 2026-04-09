use crate::errors::Error;
use crate::task::executable::RunMode;
use crate::task::measure::Measurable;
use std::time::Duration;
use thiserror::Error;

pub trait Benchmarkable: Measurable {
    fn benchmarks(&self) -> Vec<BenchmarkKind>;

    fn run_benchmarks(&mut self) -> Result<Vec<BenchmarkResult>, Error> {
        let mut results = Vec::new();

        // Not benchmarking if there are not supported methods
        for benchmark in &self.benchmarks() {
            let mode = RunMode::from(benchmark);
            if !self.supported_modes().contains(&mode) {
                return Err(Error::Benchmark(BenchmarkError::MethodIsNotSupported(mode)));
            }
        }

        for benchmark in self.benchmarks() {
            let duration = self.measure(benchmark.clone())?;
            let result = BenchmarkResult {
                kind: benchmark,
                time: duration,
            };
            results.push(result);
        }

        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub enum BenchmarkKind {
    RaceCondition,
    Deadlock,
    MutexFixed,
}

pub struct BenchmarkResult {
    pub kind: BenchmarkKind,
    pub time: Duration,
}

#[derive(Debug, Error)]
pub enum BenchmarkError {
    #[error("Method ({0}) is not supported.")]
    MethodIsNotSupported(RunMode),
}

impl From<&BenchmarkKind> for RunMode {
    fn from(kind: &BenchmarkKind) -> Self {
        match kind {
            BenchmarkKind::RaceCondition => Self::RaceCondition,
            BenchmarkKind::Deadlock => Self::Deadlock,
            BenchmarkKind::MutexFixed => Self::MutexFixed,
        }
    }
}

impl std::fmt::Display for BenchmarkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind = match self {
            Self::RaceCondition => String::from("Race Condition"),
            Self::Deadlock => String::from("Deadlock"),
            Self::MutexFixed => String::from("Mutex (Fixed)"),
        };

        write!(f, "{}", kind)
    }
}

impl std::fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {:.4} seconds.", self.kind, self.time.as_secs_f64())
    }
}
