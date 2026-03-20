use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::{Gamma, Normal};

use crate::Bandit;

#[derive(Clone, Debug)]
struct Arm {
    nu: f64,
    alpha: f64,
    beta: f64,
    reward_sum: f64,
}

impl Default for Arm {
    fn default() -> Self {
        Arm {
            nu: 1.0,
            alpha: 0.5,
            beta: 0.25,
            reward_sum: 0.0,
        }
    }
}

impl Arm {
    pub fn reward_mean(&self) -> f64 {
        self.reward_sum / self.nu
    }

    pub fn mu(&self) -> f64 {
        let d = Arm::default();
        return (d.reward_mean() * d.nu + self.reward_mean() * self.nu) / (d.nu + self.nu);
    }
}

pub struct VarTS {
    arms: Vec<Arm>,
}

impl VarTS {
    pub fn new(num_arms: usize) -> Self {
        VarTS {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl std::fmt::Display for VarTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VarTS")
    }
}

impl Bandit for VarTS {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let gamma = Gamma::new(self.arms[*i].alpha, self.arms[*i].beta);
                let lambda = gamma.unwrap().sample(rng).max(1e-4);
                let normal = Normal::new(
                    self.arms[*i].mu(),
                    (1.0 / (self.arms[*i].nu as f64 * lambda)).sqrt(),
                )
                .unwrap();
                let sample = normal.sample(rng);

                OrderedFloat(sample)
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        let r = if reward { 1.0 } else { 0.0 };
        let d = Arm::default();
        let a = &mut self.arms[arm];

        let beta_term =
            0.5 * (d.nu * a.nu) * (a.reward_mean() - d.reward_mean()).powi(2) / (d.nu + a.nu);

        a.nu += 1.0;
        a.reward_sum += r;
        a.beta -= beta_term;
        a.beta += 0.5 * (r - a.reward_mean()).powi(2);
        a.beta += 0.5 * (d.nu * a.nu) * (a.reward_mean() - d.reward_mean()).powi(2) / (d.nu + a.nu);
        a.alpha += 0.5;
    }
}
