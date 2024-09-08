use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct BrezziLaiApprox {
    arms: Vec<Arm>,
    beta: f64,
}

fn phi(s: f64) -> f64 {
    if s <= 0.2 {
        return (s / 2.0).sqrt();
    }

    if s <= 1.0 {
        return 0.49 - 0.11 * s.powf(-0.5);
    }

    if s <= 5.0 {
        return 0.63 - 0.26 * s.powf(-0.5);
    }

    if s <= 15.0 {
        return 0.77 - 0.58 * s.powf(-0.5);
    }

    (2.0 * s.ln() - s.ln().ln() - (16.0 * std::f64::consts::PI).ln()).powf(0.5)
}

impl BrezziLaiApprox {
    pub fn new(num_arms: usize, beta: f64) -> Self {
        BrezziLaiApprox {
            arms: vec![Arm::default(); num_arms],
            beta,
        }
    }
}

impl Bandit for BrezziLaiApprox {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let arm = &self.arms[*i];
                let p = 1.0 + arm.successes as f64;
                let q = 1.0 + arm.failures as f64;
                let n = p + q;
                let mu = p / n;
                let c = (self.beta.powi(-1)).ln();

                let index = ((mu * (1.0 - mu)) / (n + 1.0)).sqrt();
                let index = index * phi(1.0 / ((n + 1.0) * c));
                let index = mu + index;

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
