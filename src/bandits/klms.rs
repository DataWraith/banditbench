use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Gumbel;

use super::Arm;
use super::Bandit;

fn rel_entropy(x: f64, y: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }

    if y > 0.0 {
        return x * (x / y).ln();
    }

    10_000.0 // Infinity
}

fn kl_div(p: f64, q: f64) -> f64 {
    rel_entropy(p, q) + rel_entropy(1.0 - p, 1.0 - q)
}

pub struct KLMS {
    arms: Vec<Arm>,
}

impl KLMS {
    pub fn new(num_arms: usize) -> Self {
        Self {
            arms: vec![Arm::default(); num_arms],
        }
    }

    fn best_empirical_mean(&self) -> f64 {
        self.arms
            .iter()
            .map(|a| OrderedFloat(a.mean()))
            .max()
            .unwrap()
            .0
    }

    pub fn logits(&self) -> Vec<f64> {
        assert_ne!(self.arms.len(), 0);

        let u_max = self.best_empirical_mean();

        self.arms
            .iter()
            .map(|a| {
                let n = a.n();

                if n == 0 {
                    return 0.0;
                }

                let u = a.mean();
                let kl_div = kl_div(u, u_max);

                -(n as f64) * kl_div
            })
            .collect()
    }
}

impl Bandit for KLMS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        let gumbel_dist = Gumbel::new(0.0, 1.0).unwrap();

        self.logits()
            .iter()
            .enumerate()
            .max_by_key(|(_, l)| OrderedFloat(*l + gumbel_dist.sample(&mut rng)))
            .unwrap()
            .0
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }
    }
}
