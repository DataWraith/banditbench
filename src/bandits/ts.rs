use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Beta;

use super::Arm;
use super::Bandit;

pub struct TS {
    arms: Vec<Arm>,
}

pub struct OptimisticTS {
    arms: Vec<Arm>,
}

impl TS {
    pub fn new(num_arms: usize) -> Self {
        TS {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl OptimisticTS {
    pub fn new(num_arms: usize) -> Self {
        OptimisticTS {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl Bandit for TS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let beta = Beta::new(
                    self.arms[*i].successes as f32 + 1.0,
                    self.arms[*i].failures as f32 + 1.0,
                )
                .unwrap();

                let sample = beta.sample(&mut rng);

                OrderedFloat(sample)
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

impl Bandit for OptimisticTS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let beta = Beta::new(
                    self.arms[*i].successes as f64 + 1.0,
                    self.arms[*i].failures as f64 + 1.0,
                )
                .unwrap();

                let sample = beta.sample(&mut rng).max(self.arms[*i].mean());

                (OrderedFloat(sample), rng.gen::<u32>())
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
