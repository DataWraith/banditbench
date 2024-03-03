use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::{Normal, StandardNormal};

use super::{Arm, Bandit};

#[derive(Default, Clone)]
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
    pub fn new(num_arms: usize, optimistic_init: bool) -> Self {
        let arms = if optimistic_init {
            vec![
                ReBootArm {
                    mean: 1.0,
                    sum_of_squares: 1.0,
                    s: 1,
                };
                num_arms
            ]
        } else {
            vec![ReBootArm::default(); num_arms]
        };

        ReBoot { t: 0, arms }
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

pub struct ReBootSlow {
    arms: Vec<Arm>,
    t: usize,
}

impl ReBootSlow {
    pub fn new(num_arms: usize) -> Self {
        ReBootSlow {
            t: 0,
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl Bandit for ReBootSlow {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                let s = (self.arms[*i].successes + self.arms[*i].failures) as f64;
                let y = self.arms[*i].successes as f64 / s;

                let var = y * (1.0 - y);
                let std = var.sqrt();
                let sigma_a = std * 1.5;

                let mut weighted = 0f64;

                let w1: f64 = StandardNormal.sample(&mut rng);
                let w2: f64 = StandardNormal.sample(&mut rng);

                let w_pos = (s + 2.0).sqrt() * sigma_a;
                let w_neg = (s + 2.0).sqrt() * sigma_a * -1.0;

                weighted += w1 * w_pos;
                weighted += w2 * w_neg;

                for _ in 0..self.arms[*i].successes {
                    let w: f64 = StandardNormal.sample(&mut rng);
                    weighted += w * (1.0 - y);
                }

                for _ in 0..self.arms[*i].failures {
                    let w: f64 = StandardNormal.sample(&mut rng);
                    weighted += w * (0.0 - y);
                }

                (OrderedFloat(y + weighted / (s + 2.0)), rng.gen::<u32>())
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }

        self.t += 1;
    }
}
