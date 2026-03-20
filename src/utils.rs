use ordered_float::OrderedFloat;

/// Computes the median and Median Absolute Deviation (MAD) of a dataset.
/// 
/// MAD is a robust measure of statistical dispersion, calculated as the median
/// of the absolute deviations from the dataset's median.
/// 
/// Returns (median, mad) tuple.
pub fn median_mad(values: &[f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }
    
    let mut sorted: Vec<f64> = values.to_vec();
    sorted.sort_by_key(|&x| OrderedFloat(x));
    
    let median = sorted[sorted.len() / 2];
    
    let deviations: Vec<f64> = sorted.iter().map(|&x| (x - median).abs()).collect();
    let mut sorted_devs = deviations.clone();
    sorted_devs.sort_by_key(|&x| OrderedFloat(x));
    let mad = sorted_devs[sorted_devs.len() / 2];
    
    (median, mad)
}

pub fn softmax(input: &[f64]) -> Vec<f64> {
    let max = input.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp_sum: f64 = input.iter().map(|&x| (x - max).exp()).sum();
    input.iter().map(|&x| (x - max).exp() / exp_sum).collect()
}

/// Creates a tuple for deterministic tie-breaking.
/// Returns (OrderedFloat(value), tie_breaker) which can be used with max_by_key/min_by_key.
/// Since f64 doesn't implement Ord, OrderedFloat is used for comparison.
/// The tie_breaker (typically a random u32) ensures consistent ordering when values are equal.
pub fn tie_break(value: f64, tie_breaker: u32) -> (OrderedFloat<f64>, u32) {
    (OrderedFloat(value), tie_breaker)
}
