use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct CODE {
    arms: Vec<Arm>,
    t: usize,
    delta: f64,
}

impl CODE {
    pub fn new(num_arms: usize, delta: f64) -> CODE {
        CODE {
            t: 0,
            arms: vec![Arm::default(); num_arms],
            delta,
        }
    }

    fn confidence_bound(&self, arm: usize) -> f64 {
        ((2.0 * (1.0 / self.delta).ln()) / (self.arms[arm].n() as f64)).sqrt()
    }
}

impl Bandit for CODE {
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

        let mut best_arm = None;
        let mut best_t = (usize::MAX, 0);

        'outer: for arm in 0..self.arms.len() {
            for j in 0..self.arms.len() {
                if arm == j {
                    continue;
                }

                let lcb = self.arms[j].mean() - self.confidence_bound(j);
                let ucb = self.arms[arm].mean() + self.confidence_bound(arm);

                if ucb < lcb {
                    continue 'outer;
                }
            }

            let tie_breaker = rng.gen::<u32>();

            if best_arm.is_none() || (self.arms[arm].n(), tie_breaker) < best_t {
                best_arm = Some(arm);
                best_t = (self.arms[arm].n(), tie_breaker);
            }
        }

        best_arm.unwrap()
    }
}
