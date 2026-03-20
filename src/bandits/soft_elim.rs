use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Gumbel;

use super::Arm;
use super::Bandit;

pub struct SoftElim {
    arms: Vec<Arm>,
    theta: f64,
}

impl SoftElim {
    pub fn new(num_arms: usize, theta: f64) -> Self {
        SoftElim {
            arms: vec![Arm::default(); num_arms],
            theta,
        }
    }
}

impl std::fmt::Display for SoftElim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SoftElim (theta={:.2})", self.theta)
    }
}

impl Bandit for SoftElim {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        let gumbel = Gumbel::new(0.0, 1.0).unwrap();

        let best_mean = self
            .arms
            .iter()
            .map(|arm| OrderedFloat(arm.mean()))
            .max()
            .unwrap()
            .0;

        (0..self.arms.len())
            .max_by_key(|i| {
                let s = 2.0 * (best_mean - self.arms[*i].mean()).powi(2) * self.arms[*i].n() as f64;
                let g = gumbel.sample(rng);

                OrderedFloat(-s * self.theta + g)
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
    }
}
