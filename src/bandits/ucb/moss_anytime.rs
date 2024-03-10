use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct MOSSAnytime {
    arms: Vec<Arm>,
    t: usize,
    alpha: f64,
}

impl MOSSAnytime {
    pub fn new(num_arms: usize, alpha: f64) -> MOSSAnytime {
        MOSSAnytime {
            t: 0,
            arms: vec![Arm::default(); num_arms],
            alpha,
        }
    }
}

impl Bandit for MOSSAnytime {
    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }

        self.t += 1;
    }

    fn pull(&mut self, _rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                let m = self.arms[*i].mean();
                let cb = (((1.0 + self.alpha) / 2.0)
                    * ((self.t as f64 / ((self.arms.len() as f64) * (self.arms[*i].n() as f64)))
                        .ln()
                        .max(0.0)
                        / self.arms[*i].n() as f64))
                    .sqrt();

                OrderedFloat(m + cb)
            })
            .unwrap()
    }
}
