use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct Greedy {
    arms: Vec<Arm>,
}

impl Greedy {
    pub fn new(num_arms: usize) -> Self {
        Greedy {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl Bandit for Greedy {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                (
                    (self.arms[*i].successes + self.arms[*i].failures == 0),
                    OrderedFloat(
                        self.arms[*i].successes as f64
                            / (self.arms[*i].successes + self.arms[*i].failures) as f64,
                    ),
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
    }
}
