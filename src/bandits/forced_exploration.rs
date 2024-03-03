use ordered_float::OrderedFloat;
use rand::prelude::*;

use super::Arm;
use super::Bandit;

pub struct ForcedExploration {
    arms: Vec<Arm>,
    p: Vec<usize>,
    flags: Vec<bool>,
    r: usize,
}

impl ForcedExploration {
    pub fn new(num_arms: usize) -> Self {
        ForcedExploration {
            arms: vec![Arm::default(); num_arms],
            p: vec![0; num_arms],
            flags: vec![false; num_arms],
            r: 1,
        }
    }
}

impl Bandit for ForcedExploration {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        // Greedy choice?
        if (0..self.arms.len()).all(|i| self.p[i] < self.r) {
            return (0..self.arms.len())
                .max_by_key(|i| {
                    (
                        OrderedFloat(
                            self.arms[*i].successes as f64
                                / (self.arms[*i].successes + self.arms[*i].failures).max(1) as f64,
                        ),
                        rng.gen::<u32>(),
                    )
                })
                .unwrap();
        }

        // Forced Exploration
        (0..self.arms.len()).max_by_key(|i| self.p[*i]).unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        // Update the estimate of arm mean
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }

        // Reset forced exploration params
        self.p[arm] = 0;
        self.flags[arm] = true;

        // Increase unpulled count for unpulled arms
        for i in 0..self.arms.len() {
            if i != arm {
                self.p[i] += 1;
            }
        }

        // Update round and reset forced exploration
        if self.flags.iter().all(|&x| x) {
            self.r += 1;
            self.flags = vec![false; self.arms.len()];
        }
    }
}
