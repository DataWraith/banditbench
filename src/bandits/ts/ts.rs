use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Beta;

use crate::bandits::Arm;
use crate::Bandit;

pub struct TS {
    arms: Vec<Arm>,
}

impl TS {
    pub fn new(num_arms: usize) -> Self {
        TS {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl std::fmt::Display for TS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Thompson Sampling")
    }
}

impl Bandit for TS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let beta = Beta::new(
                    self.arms[*i].successes as f32 + 1.0,
                    self.arms[*i].failures as f32 + 1.0,
                )
                .unwrap();

                let sample = beta.sample(&mut rng);

                OrderedFloat(sample)
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        self.arms[arm].update(reward);
    }
}
