use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct BayesUCB {
    arms: Vec<Arm>,
    delta: f64,
}

impl BayesUCB {
    pub fn new(num_arms: usize, delta: f64) -> Self {
        BayesUCB {
            arms: vec![Arm::default(); num_arms],
            delta,
        }
    }
}

impl std::fmt::Display for BayesUCB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BayesUCB (δ={:.3})", self.delta)
    }
}

impl Bandit for BayesUCB {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let arm = &self.arms[*i];
                let a = 1.0 + arm.successes as f64;
                let b = 1.0 + arm.failures as f64;
                let theta = a / (a + b);
                let c = ((1.0 / self.delta).ln() / (2.0 * (a + b + 1.))).sqrt();

                (OrderedFloat(theta + c), rng.gen::<u32>())
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
    }
}
