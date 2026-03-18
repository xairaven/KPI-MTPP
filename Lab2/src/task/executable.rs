use crate::errors::Error;
use crate::task::benchmark::BenchmarkKind;

pub trait Executable {
    fn supported_methods(&self) -> Vec<ParallelismMethod>;
    fn run(&self, kind: BenchmarkKind) -> Result<(), Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParallelismMethod {
    MapReduce,
    ForkJoin,
    WorkerPool,
    Pipeline,
    ProducerConsumer,
}

impl std::fmt::Display for ParallelismMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let method = match self {
            Self::MapReduce => "Map-Reduce",
            Self::ForkJoin => "Fork-Join",
            Self::WorkerPool => "Worker Pool",
            Self::Pipeline => "Pipeline",
            Self::ProducerConsumer => "Producer-Consumer",
        };

        write!(f, "{}", method)
    }
}
