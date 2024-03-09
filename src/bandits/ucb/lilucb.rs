use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub const EPSILON: f64 = 0.0;
pub const BETA: f64 = 0.5;

pub struct LilUCB {
    arms: Vec<Arm>,
    t: usize,
    delta: f64,
}

impl LilUCB {
    pub fn new(num_arms: usize, delta: f64) -> LilUCB {
        LilUCB {
            t: 0,
            arms: vec![Arm::default(); num_arms],
            delta,
        }
    }
}

impl Bandit for LilUCB {
    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }

        self.t += 1;
    }

    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                let n_j = (self.arms[*i].successes + self.arms[*i].failures) as f64;
                let mean = self.arms[*i].successes as f64 / n_j;
                let sigma2 = 0.25;

                let cb = (1.0 + BETA) * (1.0 + EPSILON.sqrt());
                let cb = cb
                    * ((2.0
                        * sigma2
                        * (1.0 + EPSILON)
                        * (((1.0 + EPSILON) * n_j).ln() / self.delta).ln())
                        / n_j)
                        .sqrt();

                (OrderedFloat(mean + cb), rng.gen::<u32>())
            })
            .unwrap()
    }
}
