use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

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

pub struct KLUCB {
    t: usize,
    arms: Vec<Arm>,
}

impl KLUCB {
    pub fn new(num_arms: usize) -> Self {
        Self {
            t: 0,
            arms: vec![Arm::default(); num_arms],
        }
    }

    pub fn index(&self, successes: f64, failures: f64) -> f64 {
        let t = self.t as f64;
        let n = successes + failures;
        let m = successes / n;
        let limit = t.ln(); // + 3.0 * t.ln().ln();

        let mut low = m;
        let mut high = 1.0;

        while high - low > 1e-9 {
            let mid = (low + high) / 2.0;
            let kl = kl_div(m, mid);

            if n * kl > limit {
                high = mid;
            } else {
                low = mid;
            }
        }

        (low + high) / 2.0
    }
}

impl Bandit for KLUCB {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                (
                    OrderedFloat(self.index(
                        self.arms[*i].successes as f64,
                        self.arms[*i].failures as f64,
                    )),
                    rng.gen::<u32>(),
                )
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
