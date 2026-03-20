use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct OptimisticTS {
    arms: Vec<Arm>,
}

impl OptimisticTS {
    pub fn new(num_arms: usize) -> Self {
        OptimisticTS {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl std::fmt::Display for OptimisticTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Optimistic Thompson Sampling")
    }
}

impl Bandit for OptimisticTS {
    fn pull(&mut self, mut rng: &mut impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let sample = self.arms[*i].beta().sample(&mut rng).max(self.arms[*i].mean());

                (OrderedFloat(sample), rng.gen::<u32>())
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
    }
}
