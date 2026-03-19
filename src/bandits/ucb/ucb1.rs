use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct UCB1 {
    arms: Vec<Arm>,
    t: usize,
}

impl UCB1 {
    pub fn new(num_arms: usize) -> UCB1 {
        UCB1 {
            t: 0,
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl Bandit for UCB1 {
    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        self.arms[arm].update(reward);

        self.t += 1;
    }

    fn pull(&mut self, _rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                if self.arms[*i].successes + self.arms[*i].failures == 0 {
                    return OrderedFloat(f64::INFINITY);
                }

                let n_j = self.arms[*i].n() as f64;
                let ucb = self.arms[*i].mean() + (2.0 * (self.t as f64).ln() / n_j).sqrt();

                OrderedFloat(ucb)
            })
            .unwrap()
    }
}
