use rand::prelude::*;

pub const HORIZON: usize = 500;
pub const NUM_ARMS: usize = 10;
pub const NUM_RUNS: usize = 10001; // Odd, to simplify median computation

use banditbench::evaluate::evaluate_all_bandits;
use rand_distr::Beta;

fn beta_arms(seed: u64) -> Vec<f64> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let dist = Beta::new(1.0, 8.0).unwrap();

    (0..NUM_ARMS).map(|_| dist.sample(&mut rng)).collect()
}

fn main() {
    evaluate_all_bandits(NUM_RUNS, beta_arms, HORIZON)
}
