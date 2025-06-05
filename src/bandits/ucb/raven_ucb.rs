use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::Bandit;

#[derive(Default, Clone)]
pub struct RavenArm {
    pub n: usize,
    pub mean: f64,
    pub variance: f64,
}

pub struct RavenUCB {
    arms: Vec<RavenArm>,
    alpha0: f64,
    beta0: f64,
    epsilon: f64,
    t: usize,
}

impl RavenUCB {
    pub fn new(num_arms: usize, a0: f64, b0: f64, eps: f64) -> RavenUCB {
        RavenUCB {
            t: 1,
            arms: vec![RavenArm::default(); num_arms],
            alpha0: a0,
            beta0: b0,
            epsilon: eps,
        }
    }
}

impl Bandit for RavenUCB {
    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        let r = if reward { 1.0 } else { 0.0 };

        let arm = self.arms.get_mut(arm).unwrap();
        let old_mean = arm.mean;

        arm.n += 1;
        arm.mean += (r - arm.mean) / arm.n as f64;

        if arm.n > 1 {
            arm.variance += (r - old_mean) * (r - arm.mean);
            arm.variance /= (arm.n - 1) as f64;
        } else {
            arm.variance = 0.0;
        }

        self.t += 1;
    }

    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t <= self.arms.len() {
            return self.t - 1;
        }

        let alpha_t = self.alpha0 / (self.epsilon + self.t as f64).log10();

        (0..self.arms.len())
            .max_by_key(|i| {
                let arm = &self.arms[*i];
                let mean = arm.mean;
                let exploration_term = (((self.t + 1) as f64).ln() / (arm.n as f64 + 1.0)).sqrt();
                let variance_term = (arm.variance / (arm.n as f64 + 1.0) + self.epsilon).sqrt();

                (
                    OrderedFloat(mean + alpha_t * exploration_term + self.beta0 * variance_term),
                    rng.gen::<u32>(),
                )
            })
            .unwrap()
    }
}
