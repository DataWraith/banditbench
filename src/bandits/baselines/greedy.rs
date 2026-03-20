use rand::prelude::*;

use crate::bandits::Arm;
use crate::utils::tie_break;
use crate::Bandit;

pub struct Greedy {
    arms: Vec<Arm>,
}

impl Greedy {
    pub fn new(num_arms: usize) -> Self {
        Greedy {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl std::fmt::Display for Greedy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Greedy")
    }
}

impl Bandit for Greedy {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
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
