use rand::prelude::*;

use crate::bandits::Arm;
use crate::utils::tie_break;
use crate::Bandit;

pub struct EpsilonGreedy {
    arms: Vec<Arm>,
    epsilon: f64,
}

impl EpsilonGreedy {
    pub fn new(num_arms: usize, epsilon: f64) -> Self {
        EpsilonGreedy {
            arms: vec![Arm::default(); num_arms],
            epsilon,
        }
    }
}

impl std::fmt::Display for EpsilonGreedy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ϵ-Greedy (ϵ={:.3})", self.epsilon)
    }
}

impl Bandit for EpsilonGreedy {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        if rng.gen_bool(self.epsilon) {
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
    }
}
