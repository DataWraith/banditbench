use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Normal;

// Vanilla Residual Bootstrap (from the ReBoot paper), but with optimistic
// initialization to encourage exploration (similar to BDS and NPTS).

use crate::Bandit;

#[derive(Default, Clone)]
struct VResBootArm {
    mean: f64,
    sum_of_squares: f64,
    s: usize,
}

pub struct VResBoot {
    arms: Vec<VResBootArm>,
    t: usize,
}

impl VResBoot {
    pub fn new(num_arms: usize, init: usize) -> Self {
        VResBoot {
            t: 0,
            arms: vec![
                VResBootArm {
                    mean: 1.0 as f64,
                    sum_of_squares: init as f64,
                    s: init
                };
                num_arms
            ],
        }
    }
}

impl Bandit for VResBoot {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                let s = self.arms[*i].s as f64;
                let y = self.arms[*i].mean;
                let rss = self.arms[*i].sum_of_squares - s * y * y;

                let d = Normal::new(y, (s.powi(-2) * rss).sqrt()).unwrap();

                (OrderedFloat(d.sample(&mut rng)), rng.gen::<u32>())
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
