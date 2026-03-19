use rand::prelude::*;

use crate::Bandit;

pub struct Random {
    num_arms: usize,
}

impl Random {
    pub fn new(num_arms: usize) -> Self {
        Random { num_arms }
    }
}

impl std::fmt::Display for Random {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Random")
    }
}

impl Bandit for Random {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        rng.gen_range(0..self.num_arms)
    }

    fn update(&mut self, _arm: usize, _reward: bool, _rng: impl Rng) {}
}
