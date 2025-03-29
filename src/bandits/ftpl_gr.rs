use ordered_float::OrderedFloat;
use rand::prelude::*;

use super::Bandit;

pub struct FTPLGR {
    lr: f64,
    losses: Vec<f64>,
}

impl FTPLGR {
    pub fn new(num_arms: usize, lr: f64) -> Self {
        Self {
            lr,
            losses: vec![0.0; num_arms],
        }
    }

    fn sample_perturbation(&self, mut rng: impl Rng) -> Vec<f64> {
        let frechet = rand_distr::Frechet::new(0.0, 1.0, 2.0).unwrap();

        (0..self.losses.len())
            .map(|_| frechet.sample(&mut rng))
            .collect()
    }

    fn sample_index(&self, perturbation: &[f64]) -> usize {
        (0..self.losses.len())
            .min_by_key(|&i| OrderedFloat(self.losses[i] - perturbation[i] / self.lr))
            .unwrap()
    }
}

impl Bandit for FTPLGR {
    fn pull(&mut self, rng: impl Rng) -> usize {
        let perturbation = self.sample_perturbation(rng);
        self.sample_index(&perturbation)
    }

    fn update(&mut self, arm: usize, reward: bool, mut rng: impl Rng) {
        let mut m = 0;

        loop {
            m += 1;
            let perturbation = self.sample_perturbation(&mut rng);
            let idx = self.sample_index(&perturbation);

            if arm == idx {
                break;
            }
        }

        let loss = if reward { 0.0 } else { 1.0 };
        let w_inv = m as f64;

        self.losses[arm] += loss * w_inv;
    }
}
