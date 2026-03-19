use rand::prelude::*;

use super::Arm;
use super::Bandit;

fn softmax(input: &[f64]) -> Vec<f64> {
    let max = input.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp_sum: f64 = input.iter().map(|&x| (x - max).exp()).sum();
    input.iter().map(|&x| (x - max).exp() / exp_sum).collect()
}

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
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        let policy = softmax(&self.arms);

        // Sample a random entry from the policy
        let mut r = rng.gen::<f64>();
        let mut arm = 0;

        while arm < self.arms.len() {
            r -= policy[arm];
            if r <= 0.0 {
                break;
            }
            arm += 1;
        }

        arm = arm.min(self.arms.len() - 1);
        arm
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
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
