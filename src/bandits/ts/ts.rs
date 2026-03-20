use ordered_float::OrderedFloat;
use rand::prelude::*;

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
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| OrderedFloat(self.arms[*i].beta().sample(rng) as f32))
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
    }
}
