use ordered_float::OrderedFloat;

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
