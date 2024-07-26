use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Beta;

use super::Arm;
use super::Bandit;

pub struct EpsTSUCB {
    arms: Vec<Arm>,
    num_samples: usize,
    t: usize,
}

impl EpsTSUCB {
    pub fn new(num_arms: usize, num_samples: usize) -> Self {
        EpsTSUCB {
            num_samples,
            arms: vec![Arm::default(); num_arms],
            t: 0,
        }
    }
}

impl Bandit for EpsTSUCB {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        if rng.gen_bool(1.0 - 1.0 / self.arms.len() as f64) {
            return self
                .arms
                .iter()
                .enumerate()
                .max_by_key(|(_, arm)| (OrderedFloat(arm.mean()), rng.gen::<u32>()))
                .unwrap()
                .0;
        }

        let mut fts = 0f64;

        let distributions = self
            .arms
            .iter()
            .map(|arm| Beta::new(arm.successes as f64 + 1.0, arm.failures as f64 + 1.0).unwrap())
            .collect::<Vec<Beta<f64>>>();

        for _ in 0..self.num_samples {
            let mut best_sample = f64::NEG_INFINITY;

            for i in 0..self.arms.len() {
                let sample = distributions[i].sample(&mut rng);
                best_sample = best_sample.max(sample);
            }

            fts += best_sample;
        }

        let ft = fts / self.num_samples as f64;

        (0..self.arms.len())
            .min_by_key(|i| {
                OrderedFloat(f64::sqrt(self.arms[*i].n() as f64) * (ft - self.arms[*i].mean()))
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
