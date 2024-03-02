use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Beta;

use super::Arm;
use super::Bandit;

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

impl Bandit for TSVHA {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        let arm_means = self
            .arms
            .iter()
            .map(|a| {
                OrderedFloat(
                    (a.successes as f32) / (a.successes as f32 + a.failures as f32).max(1.0),
                )
            })
            .collect::<Vec<OrderedFloat<f32>>>();

        let best_mean = arm_means.iter().max().unwrap();
        let worst_mean = arm_means.iter().min().unwrap();
        let second_best = arm_means
            .iter()
            .filter(|&x| x != best_mean)
            .max()
            .unwrap_or(best_mean);

        let num_samples = 1f32
            .max(self.t as f32 * (best_mean.0 - second_best.0))
            .floor() as usize;

        (0..self.arms.len())
            .max_by_key(|&i| {
                let beta = Beta::new(
                    self.arms[i].successes as f32 + 1.0,
                    self.arms[i].failures as f32 + 1.0,
                )
                .unwrap();

                let mut samples = Vec::with_capacity(num_samples);

                for _ in 0..num_samples {
                    let sample = beta.sample(&mut rng);
                    samples.push(sample);
                }

                let sample_mean = samples.iter().sum::<f32>() / num_samples as f32;
                let combined = sample_mean.max(worst_mean.0);

                OrderedFloat(combined)
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }

        self.t += 1;
    }
}
