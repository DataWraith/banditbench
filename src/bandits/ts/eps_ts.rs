use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Beta;

use crate::bandits::Arm;
use crate::Bandit;

pub struct EpsTS {
    arms: Vec<Arm>,
}

impl EpsTS {
    pub fn new(num_arms: usize) -> Self {
        EpsTS {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl Bandit for EpsTS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if rng.gen_bool(1.0 / self.arms.len() as f64) {
            return (0..self.arms.len())
                .max_by_key(|i| {
                    let beta = Beta::new(
                        self.arms[*i].successes as f32 + 1.0,
                        self.arms[*i].failures as f32 + 1.0,
                    )
                    .unwrap();

                    let sample = beta.sample(&mut rng);

                    OrderedFloat(sample)
                })
                .unwrap();
        };

        (0..self.arms.len())
            .max_by_key(|i| {
                (
                    OrderedFloat(
                        self.arms[*i].successes as f32
                            / (self.arms[*i].successes + self.arms[*i].failures).max(1) as f32,
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
