use rand::prelude::*;

use crate::bandits::Arm;
use crate::utils::tie_break;
use crate::Bandit;

pub struct EpsilonDecreasing {
    arms: Vec<Arm>,
    epsilon: f64,
    t: usize,
}

impl EpsilonDecreasing {
    pub fn new(num_arms: usize, epsilon: f64) -> Self {
        EpsilonDecreasing {
            arms: vec![Arm::default(); num_arms],
            epsilon,
            t: 1,
        }
    }
}

impl std::fmt::Display for EpsilonDecreasing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ϵ-Decreasing (ϵ={:.3})", self.epsilon)
    }
}

impl Bandit for EpsilonDecreasing {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        if rng.gen_bool(1.0 / (self.t as f64).powf(self.epsilon)) {
            return rng.gen_range(0..self.arms.len());
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                (
                    self.arms[*i].n() == 0,
                    tie_break(self.arms[*i].mean(), rng.gen::<u32>()),
                )
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
        self.t += 1;
    }
}
