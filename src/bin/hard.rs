pub const HORIZON: usize = 500;
pub const NUM_ARMS: usize = 10;
pub const NUM_RUNS: usize = 10001; // Odd, to simplify median computation

use banditbench::evaluate::evaluate_all_bandits;

fn hard_to_distinguish_arms(seed: u64) -> Vec<f64> {
    let mut arms = vec![0.5; NUM_ARMS];
    arms[seed as usize % NUM_ARMS] += 0.01;

    arms
}

fn main() {
    evaluate_all_bandits(NUM_RUNS, hard_to_distinguish_arms, HORIZON)
}
