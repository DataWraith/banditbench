use ordered_float::OrderedFloat;
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

        let best_lcb = self
            .arms
            .iter()
            .enumerate()
            .map(|(i, arm)| arm.mean() - self.confidence_bound(i))
            .max_by_key(|x| OrderedFloat(*x))
            .unwrap();

        for arm in 0..self.arms.len() {
            let ucb = self.arms[arm].mean() + self.confidence_bound(arm);

            if ucb < best_lcb {
                continue;
            }

            let tie_breaker = rng.gen::<u32>();
            let n = self.arms[arm].n();

            if best_arm.is_none() || (n, tie_breaker) < best_t {
                best_arm = Some(arm);
                best_t = (n, tie_breaker);
            }
        }

        best_arm.unwrap()
    }
}
