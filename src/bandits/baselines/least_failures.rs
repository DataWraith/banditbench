use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct LeastFailures {
    arms: Vec<Arm>,
}

impl LeastFailures {
    pub fn new(num_arms: usize) -> Self {
        LeastFailures {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl Bandit for LeastFailures {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .min_by_key(|i| {
                (
                    self.arms[*i].failures,
                    std::cmp::Reverse(self.arms[*i].successes),
                    rng.gen::<u32>(),
                )
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }
    }
}
