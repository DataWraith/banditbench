use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Binomial;

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

impl std::fmt::Display for IRSFH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IRS.FH (H={})", self.assumed_horizon)
    }
}

impl Bandit for IRSFH {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let theta = self.arms[*i].beta().sample(rng);
                let binomial = Binomial::new(self.assumed_horizon, theta).unwrap();
                let sampled_reward = binomial.sample(rng);

                let index = (self.arms[*i].successes as f32 + sampled_reward as f32)
                    / (self.arms[*i].successes
                        + self.arms[*i].failures
                        + self.assumed_horizon as usize) as f32;

                (OrderedFloat(index), rng.gen::<u32>())
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
    }
}
