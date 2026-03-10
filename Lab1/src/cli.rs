use clap::Parser;

#[derive(Parser)]
pub struct InputArgs {
    #[arg(short,

    long,

    value_parser = clap::value_parser!(u8).range(1..=5),

    help = "Task number to execute (1-5)",

    long_help = "Specify the task number to execute.
    Valid options are strictly from 1 to 5 inclusive. Any other input will be rejected by the parser.

    Tasks:
    1) CPU-bound operation calculating Pi using the Monte Carlo method.
    2) CPU-bound operation for factoring large numbers.
    3) CPU-bound operation finding prime numbers within a specific range.
    4) Memory-bound operation that transposes a 10000x10000 matrix.
    5) I/O-bound operation involving recursive word counting in randomly generated text files."
    )]
    pub task: u8,

    // Enable `processes mode` for distributed execution.
    // Not for user -- program will use it internally when needed.
    #[arg(short, long, hide = true)]
    pub processes_mode: bool,
}
