use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Normal;

use super::Bandit;

#[derive(Default, Clone)]
struct ReBootArm {
    mean: f64,
    sum_of_squares: f64,
    s: usize,
}

pub struct ReBoot {
    arms: Vec<ReBootArm>,
    t: usize,
    r: f64,
}

impl ReBoot {
    pub fn new(num_arms: usize, r: f64) -> Self {
        ReBoot {
            t: 0,
            arms: vec![ReBootArm::default(); num_arms],
            r,
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

                let var = if y == 0.0 || y == 1.0 {
                    // Variance of a bernoulli distribution is at most 1/4
                    1.0 / 4f64
                } else {
                    self.arms[*i].sum_of_squares / s - y.powi(2)
                };

                let prss = 2.0 * self.r * self.r * (2.0 + s) * var;

                let d = Normal::new(y, (1.0 / (s + 2.0).powi(2)) * (rss + prss)).unwrap();

                (OrderedFloat(d.sample(&mut rng)), rng.gen::<u32>())
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
