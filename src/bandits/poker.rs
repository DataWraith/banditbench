use ordered_float::OrderedFloat;
use rand::prelude::*;
use statrs::distribution::ContinuousCDF;

use super::Bandit;

#[derive(Default, Clone)]
struct POKERArm {
    mean: f64,
    sum_of_squares: f64,
    s: usize,
}

pub struct POKER {
    arms: Vec<POKERArm>,
    horizon: usize,
    t: usize,
}

impl POKER {
    pub fn new(num_arms: usize, assumed_horizon: usize) -> Self {
        POKER {
            arms: vec![POKERArm::default(); num_arms],
            horizon: assumed_horizon,
            t: 0,
        }
    }
}

impl Bandit for POKER {
    fn pull(&mut self, _rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        let mut sorted_arms = self.arms.clone();
        sorted_arms.sort_by_key(|arm| OrderedFloat(-arm.mean));

        let sqrt_k = (sorted_arms.len() as f64).sqrt();

        let gap = sorted_arms[0].mean - sorted_arms[sqrt_k.round() as usize].mean;
        let delta = gap / sqrt_k;

        (0..self.arms.len())
            .max_by_key(|&i| {
                let s = self.arms[i].s as f64;
                let var = self.arms[i].sum_of_squares / s - self.arms[i].mean.powi(2);
                let std_dev = var.sqrt();

                let normal = statrs::distribution::Normal::new(
                    self.arms[i].mean,
                    (std_dev / s.sqrt()).sqrt().max(1e-8),
                )
                .unwrap();

                let prob = 1.0 - normal.cdf(sorted_arms[0].mean + delta);
                let p = self.arms[i].mean + prob * delta * self.horizon as f64;

                OrderedFloat(p)
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        let r = if reward { 1.0 } else { 0.0 };

        self.arms[arm].mean =
            (self.arms[arm].mean * self.arms[arm].s as f64 + r) / (self.arms[arm].s + 1) as f64;
        self.arms[arm].sum_of_squares += r * r;
        self.arms[arm].s += 1;

        self.t += 1;
    }
}
