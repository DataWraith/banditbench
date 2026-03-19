use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct EpsTS {
    arms: Vec<Arm>,
}

impl EpsTS {
    pub fn new(num_arms: usize) -> Self {
        EpsTS {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl std::fmt::Display for EpsTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ϵ-Exploring Thompson Sampling")
    }
}

impl Bandit for EpsTS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if rng.gen_bool(1.0 / self.arms.len() as f64) {
            return (0..self.arms.len())
                .max_by_key(|i| {
                    let sample = self.arms[*i].beta().sample(&mut rng);

                    OrderedFloat(sample)
                })
                .unwrap();
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                (
                    OrderedFloat(self.arms[*i].mean()),
                    rng.gen::<u32>(),
                )
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        self.arms[arm].update(reward);
    }
}
