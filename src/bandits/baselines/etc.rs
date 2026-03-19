use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct ETC {
    arms: Vec<Arm>,
    m: usize,
    t: usize,
}

impl ETC {
    pub fn new(num_arms: usize, m: usize) -> Self {
        ETC {
            arms: vec![Arm::default(); num_arms],
            m,
            t: 0,
        }
    }
}

impl std::fmt::Display for ETC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ETC (m={})", self.m)
    }
}

impl Bandit for ETC {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                (
                    self.arms[*i].n() < self.m,
                    OrderedFloat(self.arms[*i].mean()),
                    rng.gen::<u32>(),
                )
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if self.t < self.arms.len() * self.m {
            self.arms[arm].update(reward);
        }

        self.t += 1;
    }
}
