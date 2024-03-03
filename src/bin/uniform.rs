use rand::prelude::*;

pub const HORIZON: usize = 500;
pub const NUM_ARMS: usize = 10;
pub const NUM_RUNS: usize = 10001; // Odd, to simplify median computation

use banditbench::evaluate::evaluate_all_bandits;

fn uniform_arms(seed: u64) -> Vec<f64> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    (0..NUM_ARMS).map(|_| rng.gen_range(0.0..=1.0)).collect()
}

fn main() {
    evaluate_all_bandits(NUM_RUNS, uniform_arms, HORIZON)
}
