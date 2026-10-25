use super::{SampleStatisticsError, summarize_samples};

#[test]
fn empty_zero_and_signed_samples_remain_distinct() {
    assert_eq!(summarize_samples(&mut []), Ok(None));
    let zero = summarize_samples(&mut [0.0]).unwrap().unwrap();
    assert_eq!(
        (zero.count, zero.mean, zero.p50, zero.p95),
        (1, 0.0, 0.0, 0.0)
    );
    let mut samples = [3.0, -3.0, 0.0];
    let result = summarize_samples(&mut samples).unwrap().unwrap();
    assert_eq!(samples, [-3.0, 0.0, 3.0]);
    assert_eq!((result.mean, result.p50, result.p95), (0.0, 0.0, 3.0));
}

#[test]
fn medians_and_nearest_ranks_preserve_distinct_definitions() {
    let result = summarize_samples(&mut [4.0, 1.0, 3.0, 2.0])
        .unwrap()
        .unwrap();
    assert_eq!(result.count, 4);
    assert_eq!((result.mean, result.p50, result.p95), (2.5, 2.5, 4.0));
    let mut values =
        core::array::from_fn::<_, 20, _>(|index| f64::from(u32::try_from(20 - index).unwrap()));
    let result = summarize_samples(&mut values).unwrap().unwrap();
    assert!((result.mean - 10.5).abs() <= 10.5 * 4.0 * f64::EPSILON);
    assert_eq!((result.p50, result.p95), (10.5, 19.0));
}

#[test]
fn invalid_input_refuses_before_reordering() {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut samples = [2.0, 1.0, invalid];
        let before = samples.map(f64::to_bits);
        assert_eq!(
            summarize_samples(&mut samples),
            Err(SampleStatisticsError::NonFiniteSample { index: 2 })
        );
        assert_eq!(samples.map(f64::to_bits), before);
    }
}

#[test]
fn midpoint_handles_extremes_and_subnormal_values() {
    let maximum = summarize_samples(&mut [f64::MAX, f64::MAX])
        .unwrap()
        .unwrap();
    assert_eq!((maximum.mean, maximum.p50), (f64::MAX, f64::MAX));
    let mixed = summarize_samples(&mut [-f64::MAX, f64::MAX])
        .unwrap()
        .unwrap();
    assert_eq!((mixed.mean, mixed.p50), (0.0, 0.0));
    let tiny = f64::from_bits(1);
    assert_eq!(
        summarize_samples(&mut [tiny, tiny]).unwrap().unwrap().p50,
        tiny
    );
    let mut zeroes = [0.0, -0.0];
    summarize_samples(&mut zeroes).unwrap().unwrap();
    assert!(zeroes[0].is_sign_negative());
    assert!(zeroes[1].is_sign_positive());
}

#[test]
fn rounded_sum_overflow_is_explicit_after_sorting() {
    let mut samples = [f64::MAX; 3];
    assert_eq!(
        summarize_samples(&mut samples),
        Err(SampleStatisticsError::NonFiniteMean)
    );
    assert_eq!(samples, [f64::MAX; 3]);
}
