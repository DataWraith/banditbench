use rand::prelude::*;

pub const HORIZON: usize = 500;
pub const NUM_ARMS: usize = 10;
pub const NUM_RUNS: usize = 10001; // Odd, to simplify median computation

use banditbench::evaluate::evaluate_all_bandits;

fn half_range(seed: u64) -> Vec<f64> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    (0..NUM_ARMS).map(|_| rng.gen_range(0.25..=0.75)).collect()
}

fn main() {
    evaluate_all_bandits(NUM_RUNS, half_range, HORIZON)
}
