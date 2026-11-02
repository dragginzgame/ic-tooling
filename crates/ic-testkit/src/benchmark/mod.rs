//! Compact benchmark markers on every target; parsing and analysis on native hosts.

mod marker;
pub use marker::{
    BenchmarkCounters, BenchmarkEventKind, BenchmarkMarkerError, DEFAULT_PREFIX, format_marker,
};

#[cfg(not(target_arch = "wasm32"))]
mod analysis;
#[cfg(not(target_arch = "wasm32"))]
pub use analysis::{
    BenchmarkAggregateError, BenchmarkAggregateRow, BenchmarkAverageError, BenchmarkAverages,
    BenchmarkComparisonError, BenchmarkComparisonRow, BenchmarkCounter, BenchmarkEventSource,
    BenchmarkMeasurement, BenchmarkParseReport, BenchmarkParserConfig, BenchmarkScope,
    BenchmarkSpan, BenchmarkSpanReport, InvalidBenchmarkSpan, MalformedBenchmarkMarker,
    RawBenchmarkEvent, SuiteDerivation, aggregate_benchmark_spans, compare_benchmark_aggregates,
    pair_benchmark_spans, parse_benchmark_events, parse_benchmark_events_from_captured_output,
    parse_benchmark_events_from_source,
};
