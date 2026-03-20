use rand::prelude::*;
use rand_distr::Exp1;

use crate::bandits::Arm;
use crate::Bandit;

pub const ALPHA0: usize = 1;
pub const BETA0: usize = 1;

pub struct WB {
    arms: Vec<Arm>,
}

fn theta(arm: &Arm, rng: &mut impl Rng) -> f64 {
    let n = arm.n();

    let mut numerator = 0.0;
    let mut denominator = 0.0;

    for i in 0..n {
        let y = if i < arm.successes { 1.0 } else { 0.0 };
        let w: f64 = rng.sample(Exp1);
        numerator += w * y;
        denominator += w;
    }

    for _ in 0..ALPHA0 {
        let w: f64 = rng.sample(Exp1);
        numerator += w;
        denominator += w;
    }

    for _ in 0..BETA0 {
        let w: f64 = rng.sample(Exp1);
        denominator += w;
    }

    numerator / denominator
}

impl WB {
    pub fn new(num_arms: usize) -> Self {
        Self {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl std::fmt::Display for WB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Weighted Bootstrap")
    }
}

impl Bandit for WB {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        let mut best_i = 0;
        let mut best_theta = theta(&self.arms[0], rng);
        for i in 1..self.arms.len() {
            let t = theta(&self.arms[i], rng);
            if t > best_theta {
                best_theta = t;
                best_i = i;
            }
        }
        best_i
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
    }
}
