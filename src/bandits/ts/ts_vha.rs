use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct TSVHA {
    t: usize,
    arms: Vec<Arm>,
}

impl TSVHA {
    pub fn new(num_arms: usize) -> Self {
        TSVHA {
            t: 0,
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl std::fmt::Display for TSVHA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Thompson Sampling with Virtual Helping Agents (Combiner C3)")
    }
}

impl Bandit for TSVHA {
    fn pull(&mut self, mut rng: &mut impl Rng) -> usize {
        let arm_means = self
            .arms
            .iter()
            .map(|a| OrderedFloat(a.mean()))
            .collect::<Vec<OrderedFloat<f64>>>();

        let best_mean = arm_means.iter().max().unwrap();
        let worst_mean = arm_means.iter().min().unwrap();
        let second_best = arm_means
            .iter()
            .filter(|&x| x != best_mean)
            .max()
            .unwrap_or(best_mean);

        let num_samples = 1f64
            .max(self.t as f64 * (best_mean.0 - second_best.0))
            .floor() as usize;

        (0..self.arms.len())
            .max_by_key(|&i| {
                let beta = self.arms[i].beta();
                let mut samples = Vec::with_capacity(num_samples);

                for _ in 0..num_samples {
                    let sample = beta.sample(&mut rng) as f32;
                    samples.push(sample);
                }

                let sample_mean = samples.iter().sum::<f32>() / num_samples as f32;
                let combined = sample_mean.max(worst_mean.0 as f32);

                OrderedFloat(combined)
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);

        self.t += 1;
    }
}
