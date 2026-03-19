use rand::prelude::*;

use crate::utils::softmax;

use super::Bandit;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub struct DelightfulGradientBandit {
    lr: f64,
    arms: Vec<f64>,
}

impl DelightfulGradientBandit {
    pub fn new(num_arms: usize, lr: f64) -> Self {
        DelightfulGradientBandit {
            lr,
            arms: vec![0.0; num_arms],
        }
    }
}

impl std::fmt::Display for DelightfulGradientBandit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Delightful GB (lr={:.3})", self.lr)
    }
}

impl Bandit for DelightfulGradientBandit {
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

        let r = if reward { 1.0 } else { -1.0 };

        let surprisal = -policy[arm].ln();
        let gate = sigmoid(surprisal * r);

        for a in 0..self.arms.len() {
            if a == arm {
                self.arms[a] += self.lr * gate * (1.0 - policy[a]) * r;
            } else {
                self.arms[a] -= self.lr * gate * policy[a] * r;
            }
        }
    }
}
