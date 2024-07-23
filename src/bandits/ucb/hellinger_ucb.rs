use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

const EPSILON: f64 = 2e-16;
const C: f64 = 0.27;

pub struct HellingerUCB {
    arms: Vec<Arm>,
    t: usize,
}

impl HellingerUCB {
    pub fn new(num_arms: usize) -> HellingerUCB {
        HellingerUCB {
            t: 0,
            arms: vec![Arm::default(); num_arms],
        }
    }
}

fn h2_bernoulli(x: f64, y: f64) -> f64 {
    let x = x.clamp(EPSILON, 1.0 - EPSILON);
    let y = y.clamp(EPSILON, 1.0 - EPSILON);

    1.0 - (x * y).sqrt() - ((1.0 - x) * (1.0 - y)).sqrt()
}

fn h2_index(x: f64, d: f64) -> f64 {
    let mut l = x.max(-1.0);
    let mut u = 1.0;

    while (u - l) > 1e-6 {
        let m = (l + u) / 2.0;

        if h2_bernoulli(x, m) > d {
            u = m;
        } else {
            l = m;
        }
    }

    (l + u) / 2.0
}

impl Bandit for HellingerUCB {
    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }

        self.t += 1;
    }

    fn pull(&mut self, _rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                if self.arms[*i].successes + self.arms[*i].failures == 0 {
                    return OrderedFloat(f64::INFINITY);
                }

                let n_j = (self.arms[*i].successes + self.arms[*i].failures) as f64;
                let mean = self.arms[*i].successes as f64 / n_j;
                let d = 1.0 - (-C * (self.t as f64).ln() / n_j).exp();
                let index = h2_index(mean, d).min(1.0);

                OrderedFloat(index)
            })
            .unwrap()
    }
}
