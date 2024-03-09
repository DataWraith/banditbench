use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct EpsilonGreedy {
    arms: Vec<Arm>,
    epsilon: f64,
}

impl EpsilonGreedy {
    pub fn new(num_arms: usize, epsilon: f64) -> Self {
        EpsilonGreedy {
            arms: vec![Arm::default(); num_arms],
            epsilon,
        }
    }
}

impl Bandit for EpsilonGreedy {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if rng.gen_bool(self.epsilon) {
            return rng.gen_range(0..self.arms.len());
        }

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
