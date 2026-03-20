use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::utils::softmax;

use super::Arm;
use super::Bandit;

pub struct GradientBandit {
    lr: f64,
    arms: Vec<f64>,
    stats: Arm,
    use_baseline: bool,
}

impl GradientBandit {
    pub fn new(num_arms: usize, lr: f64, use_baseline: bool) -> Self {
        GradientBandit {
            lr,
            arms: vec![0.0; num_arms],
            stats: Arm::default(),
            use_baseline,
        }
    }
}

impl std::fmt::Display for GradientBandit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.use_baseline {
            write!(f, "Gradient Bandit (with baseline)")
        } else {
            write!(f, "Gradient Bandit")
        }
    }
}

impl Bandit for GradientBandit {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        let policy = softmax(&self.arms);
        let dist = WeightedIndex::new(&policy).unwrap();
        dist.sample(rng)
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        let policy = softmax(&self.arms);

        self.stats.update(reward);

        let mut r = if reward { 1.0 } else { -1.0 };

        if self.use_baseline {
            r -= self.stats.mean();
        }

        for a in 0..self.arms.len() {
            if a == arm {
                self.arms[a] += self.lr * (1.0 - policy[a]) * r;
            } else {
                self.arms[a] -= self.lr * policy[a] * r;
            }
        }
    }
}
