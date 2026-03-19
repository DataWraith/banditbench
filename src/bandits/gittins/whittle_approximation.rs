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

impl std::fmt::Display for WhittleApprox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gittins Index -- Whittle's Approximation (β={:.2})", self.beta)
    }
}

impl Bandit for WhittleApprox {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let arm = &self.arms[*i];

                let p = 1.0 + arm.successes as f64;
                let q = 1.0 + arm.failures as f64;
                let n = p + q;
                let mu = p / n;

                let c = (self.beta.powi(-1)).ln();

                let index = mu
                    + ((mu * (1.0 - mu))
                        / (n * ((2.0 * c + n.powi(-1)) * mu * (1.0 - mu)).sqrt() + mu - 0.5));

                (OrderedFloat(index), rng.gen::<u32>())
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        self.arms[arm].update(reward);
    }
}
