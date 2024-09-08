use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct WhittleApprox {
    arms: Vec<Arm>,
    beta: f64,
}

impl WhittleApprox {
    pub fn new(num_arms: usize, beta: f64) -> Self {
        WhittleApprox {
            arms: vec![Arm::default(); num_arms],
            beta,
        }
    }
}

impl Bandit for WhittleApprox {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let arm = &self.arms[*i];
                let mu = arm.mean();
                let n = arm.n() as f64;
                let c = (1.0 / self.beta).ln();
                let index = mu
                    + ((mu * (1.0 - mu))
                        / (n * ((2.0 * c + 1.0 / n) * mu * (1.0 - mu)).sqrt() + mu - 0.5));

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
