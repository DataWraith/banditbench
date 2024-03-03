use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Normal;

use super::Bandit;

#[derive(Clone, Default)]
struct ReBootArm {
    mean: f64,
    sum_of_squares: f64,
    s: usize,
}

pub struct ReBoot {
    arms: Vec<ReBootArm>,
    t: usize,
}

impl ReBoot {
    pub fn new(num_arms: usize) -> Self {
        ReBoot {
            t: 0,
            arms: vec![ReBootArm::default(); num_arms],
        }
    }
}

impl Bandit for ReBoot {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                let s = self.arms[*i].s as f64;
                let y = self.arms[*i].mean;
                let rss = self.arms[*i].sum_of_squares - s * y * y;
                let d = Normal::new(y, s.powi(-2) * rss).unwrap();

                OrderedFloat(d.sample(&mut rng))
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        let r = if reward { 1.0 } else { 0.0 };

        self.arms[arm].mean =
            (self.arms[arm].mean * self.arms[arm].s as f64 + r) / (self.arms[arm].s + 1) as f64;
        self.arms[arm].sum_of_squares += r;
        self.arms[arm].s += 1;

        self.t += 1;
    }
}
