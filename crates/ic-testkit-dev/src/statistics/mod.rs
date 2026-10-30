//! Private finite-sample projections for the fixture report.

#[cfg(test)]
mod tests;

/// Approximate floating-point projections of a complete finite sample slice.
///
/// The count is exact. Mean and p50 use floating-point arithmetic; the
/// nearest-rank p95 is an observed value, not a histogram bucket range.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
pub struct SampleStatistics {
    /// Number of admitted observations, including measured zeroes.
    pub count: usize,
    /// Sum of each observation divided by the sample count.
    pub mean: f64,
    /// Middle observation, or the midpoint of the two middle observations.
    pub p50: f64,
    /// Observed value at the nearest rank covering 95 percent of the samples.
    pub p95: f64,
}

/// Why a complete sample slice cannot supply the requested projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SampleStatisticsError {
    /// A sample is NaN or infinite; admission fails before sorting.
    #[error("sample {index} is not finite")]
    NonFiniteSample {
        /// Original zero-based position of the first invalid sample.
        index: usize,
    },
    /// Floating-point summation overflowed; the sample slice remains sorted.
    #[error("sample mean is not finite")]
    NonFiniteMean,
}

/// Sort finite samples in place and project the report's mean, p50 and nearest-rank p95.
///
/// Consumers own sample collection, units, memory and attribution. This function
/// neither samples a platform nor allocates storage.
/// Negative values and signed zeroes are admitted; empty samples return `None`.
/// The p95 rank uses `ceil(count * 95 / 100)`, computed with integer arithmetic.
/// Even-count medians use a midpoint that avoids overflow and preserves equal
/// subnormal observations.
///
/// # Errors
/// A non-finite sample refuses before changing the slice.
/// A non-finite mean returns an error after sorting. Counts remain exact, while
/// floating-point projections and the conversion of the count for division are
/// approximate. No histogram or exact-integer aggregate is reconstructed.
///
#[expect(
    clippy::cast_precision_loss,
    reason = "finite-sample means divide by an approximate floating-point count"
)]
pub fn summarize_samples(
    samples: &mut [f64],
) -> Result<Option<SampleStatistics>, SampleStatisticsError> {
    for (index, sample) in samples.iter().enumerate() {
        if !sample.is_finite() {
            return Err(SampleStatisticsError::NonFiniteSample { index });
        }
    }
    if samples.is_empty() {
        return Ok(None);
    }
    samples.sort_unstable_by(f64::total_cmp);
    let count = samples.len();
    let mean = samples
        .iter()
        .map(|sample| sample / count as f64)
        .sum::<f64>();
    if !mean.is_finite() {
        return Err(SampleStatisticsError::NonFiniteMean);
    }
    let middle = count / 2;
    let p50 = if count.is_multiple_of(2) {
        samples[middle - 1].midpoint(samples[middle])
    } else {
        samples[middle]
    };
    // usize is at most 64 bits on supported hosts; this product fits in u128.
    let rank = ((count as u128) * 95).div_ceil(100);
    let index = (rank - 1) as usize;
    Ok(Some(SampleStatistics {
        count,
        mean,
        p50,
        p95: samples[index],
    }))
}
