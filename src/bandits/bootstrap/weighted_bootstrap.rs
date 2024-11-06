use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Exp1;

use crate::bandits::Arm;
use crate::Bandit;

pub const ALPHA0: usize = 1;
pub const BETA0: usize = 1;

pub struct WB {
    arms: Vec<Arm>,
}

fn theta(arm: &Arm, mut rng: impl Rng) -> f64 {
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

impl Bandit for WB {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| OrderedFloat(theta(&self.arms[*i], &mut rng)))
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
