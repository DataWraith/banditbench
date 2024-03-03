use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Beta;

use super::Arm;
use super::Bandit;

pub struct TSUCB {
    arms: Vec<Arm>,
    num_samples: usize,
}

impl TSUCB {
    pub fn new(num_arms: usize, num_samples: usize) -> Self {
        TSUCB {
            num_samples,
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl Bandit for TSUCB {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        let mut fts = 0f32;

        for _ in 0..self.num_samples {
            let mut best_sample = f32::NEG_INFINITY;

            for i in 0..self.arms.len() {
                let beta = Beta::new(
                    self.arms[i].successes as f32 + 1.0,
                    self.arms[i].failures as f32 + 1.0,
                )
                .unwrap();

                let sample = beta.sample(&mut rng);

                best_sample = best_sample.max(sample);
            }

            fts += best_sample;
        }

        let ft = fts / self.num_samples as f32;

        (0..self.arms.len())
            .rev()
            .min_by_key(|i| {
                let w = self.arms[*i].successes;
                let l = self.arms[*i].failures;

                if w + l == 0 {
                    OrderedFloat(f32::NEG_INFINITY)
                } else {
                    OrderedFloat(f32::sqrt((w + l) as f32) * (ft - (w as f32) / (w + l) as f32))
                }
            })
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
