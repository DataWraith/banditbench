use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Gumbel;

use super::Arm;
use super::Bandit;

pub struct SoftElim {
    arms: Vec<Arm>,
    gamma: f64,
}

impl SoftElim {
    pub fn new(num_arms: usize, w: f64) -> Self {
        SoftElim {
            arms: vec![Arm::default(); num_arms],
            gamma: w.powi(-2),
        }
    }
}

impl std::fmt::Display for SoftElim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SoftElim (w={:.2})", self.gamma.sqrt())
    }
}

impl Bandit for SoftElim {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
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
                let g = gumbel.sample(&mut rng);

                OrderedFloat(-s * self.gamma + g)
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        self.arms[arm].update(reward);
    }
}
