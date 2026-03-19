pub fn softmax(input: &[f64]) -> Vec<f64> {
    let max = input.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp_sum: f64 = input.iter().map(|&x| (x - max).exp()).sum();
    input.iter().map(|&x| (x - max).exp() / exp_sum).collect()
}
