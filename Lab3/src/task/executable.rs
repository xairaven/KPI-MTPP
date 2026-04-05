use crate::errors::Error;
use crate::task::benchmark::BenchmarkKind;

pub trait Executable {
    fn supported_modes(&self) -> Vec<RunMode>;
    fn run(&self, kind: BenchmarkKind) -> Result<(), Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum RunMode {
    Sequential,
}

impl std::fmt::Display for RunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let method = match self {
            Self::Sequential => "Sequential",
        };

        write!(f, "{}", method)
    }
}
