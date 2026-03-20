use rand::prelude::*;
use rand_distr::Normal;

use crate::utils::tie_break;
use crate::Bandit;

const BOOTSTRAP_REPLICATES: usize = 150;
const EXPLORATION_LAMBDA: f64 = 0.5;
const BOOTSTRAP_SIGMA: f64 = 0.5;

fn bootstrap_weights(rng: &mut impl Rng) -> [f64; 3] {
    let d = Normal::new(1.0, BOOTSTRAP_SIGMA.sqrt()).unwrap();

    [d.sample(rng), d.sample(rng), d.sample(rng)]
}

pub struct Mbe {
    replicates: Vec<MbeMab>,
}

impl Mbe {
    pub fn new(num_arms: usize) -> Self {
        Self {
            replicates: (0..BOOTSTRAP_REPLICATES)
                .map(|_| MbeMab::new(num_arms))
                .collect(),
        }
    }
}

impl std::fmt::Display for Mbe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Multiplier Bootstrap-based Exploration")
    }
}

impl Bandit for Mbe {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        self.replicates.choose_mut(rng).unwrap().pull(rng)
    }

    fn update(&mut self, arm: usize, reward: bool, rng: &mut impl Rng) {
        for mab in &mut self.replicates {
            mab.update(arm, reward, rng);
        }
    }
}

#[derive(Default)]
struct MbeMab {
    sums: Vec<f64>,
    counts: Vec<f64>,
}

impl MbeMab {
    pub fn new(num_arms: usize) -> Self {
        Self {
            sums: vec![0.0; num_arms],
            counts: vec![0.0; num_arms],
        }
    }

    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        (0..self.sums.len())
            .max_by_key(|&c| {
                let sum = self.sums[c];
                let count = self.counts[c];

                if count > 0.0 {
                    tie_break(sum / count, rng.gen::<u32>())
                } else {
                    tie_break(f64::INFINITY, rng.gen::<u32>())
                }
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, rng: &mut impl Rng) {
        let [w1, w2, w3] = bootstrap_weights(rng);
        let r = if reward { 1.0 } else { 0.0 };

        // Update rewards
        self.sums[arm] += r * w1;
        self.counts[arm] += w1;

        // Pseudo-victory
        self.sums[arm] += 1.0 * w2 * EXPLORATION_LAMBDA;
        self.counts[arm] += w2 * EXPLORATION_LAMBDA;

        // Pseudo-loss
        self.sums[arm] += 0.0 * w3 * EXPLORATION_LAMBDA;
        self.counts[arm] += w3 * EXPLORATION_LAMBDA;
    }
}
