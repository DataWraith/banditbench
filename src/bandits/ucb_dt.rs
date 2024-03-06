use ordered_float::OrderedFloat;
use rand::prelude::*;

use super::Arm;
use super::Bandit;

pub struct UCBDT {
    arms: Vec<Arm>,
    gamma: f64,
    t: usize,
}

impl UCBDT {
    pub fn new(num_arms: usize, gamma: f64) -> UCBDT {
        UCBDT {
            t: 0,
            arms: vec![Arm::default(); num_arms],
            gamma,
        }
    }

    pub fn distance(&self, i: usize, j: usize) -> f64 {
        let mu_i = self.arms[i].mean();
        let mu_j = self.arms[j].mean();
        let diff = (mu_i - mu_j).abs();
        let n_i = self.arms[i].successes + self.arms[i].failures;

        diff.powf(1.0 / (self.gamma * n_i as f64).floor())
    }
}

impl Bandit for UCBDT {
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
                let mu = self.arms[*i].mean();
                let mut n = self.arms[*i].n() as f64;

                for j in 0..self.arms.len() {
                    if *i == j {
                        continue;
                    }

                    let d = self.distance(*i, j);
                    n += d * self.arms[j].n() as f64;
                }

                (
                    OrderedFloat(mu + (2.0 * (self.t as f64).ln() / n).sqrt()),
                    rng.gen::<u32>(),
                )
            })
            .unwrap()
    }
}
