use crate::errors::{Error, SystemError};
use crate::task::benchmark::{BenchmarkMetadata, Benchmarkable};
use crate::task::executor::Executable;
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use std::ops::RangeInclusive;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

pub struct Primes {
    range: RangeInclusive<u64>,
}

// Builder
impl Primes {
    pub fn with_range(range: RangeInclusive<u64>) -> Self {
        Self { range }
    }
}

impl Primes {
    // Core mathematical algorithm to verify if a single number is prime.
    fn is_prime(n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }

        // Eliminate multiples of 2 and 3 early to save CPU cycles.
        if n.is_multiple_of(2) || n.is_multiple_of(3) {
            return false;
        }

        let mut i = 5;
        // Check odd divisors up to the square root of n.
        while i * i <= n {
            if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
                return false;
            }
            i += 6;
        }

        true
    }

    // Helper method to count prime numbers within a specific chunk.
    fn count_primes_in_range(range: RangeInclusive<u64>) -> usize {
        let mut count = 0;
        for n in range {
            if Self::is_prime(n) {
                count += 1;
            }
        }
        count
    }
}

impl Benchmarkable for Primes {}
impl Measurable for Primes {}
impl Manageable for Primes {}

impl Reportable for Primes {
    fn name(&self) -> &'static str {
        "CPU-Bound: Primes"
    }
}

impl Executable for Primes {
    fn run_sequential(&self) -> Result<(), Error> {
        let total_primes = Self::count_primes_in_range(self.range.clone());

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Sequential,
            total_primes
        );

        Ok(())
    }

    fn run_threads(&self, threads: usize) -> Result<(), Error> {
        let start_bound = *self.range.start();
        let end_bound = *self.range.end();

        // Calculate total items and chunk size to split the range evenly.
        let total_elements = end_bound.saturating_sub(start_bound) + 1;
        let chunk_size = total_elements.div_ceil(threads as u64);

        // Shared counter to safely accumulate the total primes found by all threads.
        let total_primes = AtomicUsize::new(0);

        thread::scope(|s| {
            for i in 0..threads {
                let chunk_start = start_bound + (i as u64) * chunk_size;
                let chunk_end = std::cmp::min(end_bound, chunk_start + chunk_size - 1);

                let shared_total = &total_primes;

                s.spawn(move || {
                    if chunk_start <= chunk_end {
                        let local_count =
                            Self::count_primes_in_range(chunk_start..=chunk_end);
                        // Add the local result to the global atomic counter.
                        shared_total.fetch_add(local_count, Ordering::Relaxed);
                    }
                });
            }
        });

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Threads(threads),
            total_primes.load(Ordering::SeqCst)
        );

        Ok(())
    }

    fn run_workers(&self, workers: usize) -> Result<(), Error> {
        let start_bound = *self.range.start();
        let end_bound = *self.range.end();

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(SystemError::RayonPoolBuild)?;

        // Calculate chunks. Using more chunks than workers for better work-stealing.
        let total_elements = end_bound.saturating_sub(start_bound) + 1;
        let chunks_count = workers * 10;
        let chunk_size = total_elements.div_ceil(chunks_count as u64);

        let total_primes: usize = pool.install(|| {
            use rayon::prelude::*;

            // Map each chunk to its prime count, then sum all counts in parallel.
            (0..chunks_count)
                .into_par_iter()
                .map(|i| {
                    let chunk_start = start_bound + (i as u64) * chunk_size;
                    let chunk_end =
                        std::cmp::min(end_bound, chunk_start + chunk_size - 1);

                    if chunk_start <= chunk_end {
                        Self::count_primes_in_range(chunk_start..=chunk_end)
                    } else {
                        0
                    }
                })
                .sum()
        });

        log::info!(
            "TASK: {}, MODE: {:?}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Workers(workers),
            total_primes
        );

        Ok(())
    }

    fn run_process_chunk(
        &self, process_index: usize, total_processes: usize,
    ) -> Result<(), Error> {
        let start_bound = *self.range.start();
        let end_bound = *self.range.end();

        let total_elements = end_bound.saturating_sub(start_bound) + 1;
        let chunk_size = total_elements.div_ceil(total_processes as u64);

        let chunk_start = start_bound + (process_index as u64) * chunk_size;
        let chunk_end = std::cmp::min(end_bound, chunk_start + chunk_size - 1);

        let mut local_primes = 0;

        // Execute only the mathematical range assigned to this specific process.
        if chunk_start <= chunk_end {
            local_primes = Self::count_primes_in_range(chunk_start..=chunk_end);
        }

        // Print the localized count to stdout for the orchestrator.
        println!("{}", local_primes);

        Ok(())
    }

    fn aggregate_process_results(
        &self, total_processes: usize, results: Vec<String>,
    ) -> Result<(), Error> {
        let mut total_primes = 0;

        // Parse each process output and accumulate the total count.
        for result_str in results {
            match result_str.trim().parse::<usize>() {
                Ok(count) => {
                    total_primes += count;
                },
                Err(_) => Err(SystemError::FailedParsingChildOutput(result_str))?,
            }
        }

        log::info!(
            "TASK: {}, MODE: {:?}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Processes(total_processes),
            total_primes,
        );

        Ok(())
    }
}
