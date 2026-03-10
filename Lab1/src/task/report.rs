use crate::task::benchmark::{BenchmarkKind, BenchmarkResult, Benchmarkable};

#[derive(Debug, Default)]
pub struct TextBuffer {
    buffer: String,
}

impl TextBuffer {
    pub fn with_new_paragraph(&mut self, line: &str) {
        self.buffer.push('\n');
        self.add_line(line);
    }

    pub fn add_line(&mut self, line: &str) {
        self.buffer.push_str(line);
        self.buffer.push('\n');
    }

    pub fn text(&self) -> String {
        self.buffer.clone()
    }
}

#[derive(Debug, Default)]
pub struct Report {
    buffer: TextBuffer,
}

impl Report {
    pub fn add_header(&mut self, header: &str) {
        let header = format!("=== {} ===", header.to_ascii_uppercase().trim());
        self.buffer.with_new_paragraph(&header);
    }

    pub fn add_result(&mut self, result: &BenchmarkResult) {
        self.buffer.add_line(&result.to_string());
    }

    pub fn text(&self) -> String {
        self.buffer.text()
    }

    pub fn extend(&mut self, other: Report) {
        self.buffer.with_new_paragraph(&other.text());
    }

    pub fn by_kind(benchmarks: &[BenchmarkResult], kind: BenchmarkKind) -> Option<Self> {
        let results = BenchmarkResult::filter_by_kind(benchmarks, &kind);
        if !results.is_empty() {
            let mut report = Report::default();

            report.add_header(&kind.to_string());
            for result in results {
                report.add_result(&result);
            }

            return Some(report);
        }

        None
    }
}

pub trait Reportable: Benchmarkable {
    fn name(&self) -> &'static str;

    fn report(&mut self) -> Report {
        let mut report = Report::default();

        let benchmarks = self.benchmark();

        // Sequential
        if let Some(sequential_report) =
            Report::by_kind(&benchmarks, BenchmarkKind::Sequential)
        {
            report.extend(sequential_report);
        }

        // Threads
        if let Some(threads_report) = Report::by_kind(&benchmarks, BenchmarkKind::Threads)
        {
            report.extend(threads_report);
        }

        // Workers
        if let Some(workers_report) = Report::by_kind(&benchmarks, BenchmarkKind::Workers)
        {
            report.extend(workers_report);
        }

        // Processes
        if let Some(processes_report) =
            Report::by_kind(&benchmarks, BenchmarkKind::Processes)
        {
            report.extend(processes_report);
        }

        report
    }
}
