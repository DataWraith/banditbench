use ordered_float::OrderedFloat;
use rand::prelude::*;

use super::Arm;
use super::Bandit;

pub struct TSUCB {
    arms: Vec<Arm>,
    num_samples: usize,
    t: usize,
}

impl TSUCB {
    pub fn new(num_arms: usize, num_samples: usize) -> Self {
        TSUCB {
            num_samples,
            arms: vec![Arm::default(); num_arms],
            t: 0,
        }
    }
}

impl std::fmt::Display for TSUCB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TS-UCB ({} samples)", self.num_samples)
    }
}

impl Bandit for TSUCB {
    fn pull(&mut self, rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        let ft = self.estimate_highest_reward(rng);

        (0..self.arms.len())
            .min_by_key(|i| {
                let explore = (self.arms[*i].n() as f64).sqrt();
                let exploit = ft - self.arms[*i].mean();

                OrderedFloat(explore * exploit)
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        self.arms[arm].update(reward);
        self.t += 1;
    }
}

impl TSUCB {
    fn estimate_highest_reward(&self, mut rng: impl Rng) -> f64 {
        let mut best_samples = vec![f64::NEG_INFINITY; self.num_samples];

        for arm in self.arms.iter() {
            let distribution = arm.beta();

            best_samples
                .iter_mut()
                .zip(distribution.sample_iter(&mut rng))
                .for_each(|(f, s)| *f = f.max(s));
        }

        best_samples.iter().sum::<f64>() / self.num_samples as f64
    }
}
