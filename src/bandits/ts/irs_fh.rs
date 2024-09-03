use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::{Beta, Binomial};

use crate::bandits::Arm;
use crate::Bandit;

pub struct IRSFH {
    assumed_horizon: u64,
    arms: Vec<Arm>,
}

impl IRSFH {
    pub fn new(num_arms: usize, assumed_horizon: u64) -> Self {
        IRSFH {
            assumed_horizon,
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl Bandit for IRSFH {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let beta = Beta::new(
                    self.arms[*i].successes as f64 + 1.0,
                    self.arms[*i].failures as f64 + 1.0,
                )
                .unwrap();

                let theta = beta.sample(&mut rng);
                let binomial = Binomial::new(self.assumed_horizon, theta).unwrap();
                let sampled_reward = binomial.sample(&mut rng);

                let index = (self.arms[*i].successes as f32 + sampled_reward as f32)
                    / (self.arms[*i].successes
                        + self.arms[*i].failures
                        + self.assumed_horizon as usize) as f32;

                (OrderedFloat(index), rng.gen::<u32>())
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
