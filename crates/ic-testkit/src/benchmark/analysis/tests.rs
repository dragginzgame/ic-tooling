use super::{AggregateBuilder, BenchmarkCounter, BenchmarkCounters, BenchmarkSpan};

#[test]
fn aggregate_run_count_overflow_is_rejected_without_changing_totals() {
    let span = BenchmarkSpan {
        suite: "probe".into(),
        span_label: "probe/run".into(),
        start_line: 1,
        end_line: 2,
        end: BenchmarkCounters::default(),
        delta: BenchmarkCounters::default(),
    };
    let mut builder = AggregateBuilder::new(&span);
    builder.runs = u64::MAX;
    assert_eq!(builder.push(&span), Err(BenchmarkCounter::Runs));
    assert_eq!(builder.runs, u64::MAX);
    assert_eq!(builder.total, BenchmarkCounters::default());
}
