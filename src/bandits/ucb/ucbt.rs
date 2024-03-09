use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::Bandit;

#[derive(Clone)]
pub struct ArmStats {
    mean: f64,
    sum_of_squares: f64,
    s: usize,
}

pub struct UCBT {
    arms: Vec<ArmStats>,
}

impl UCBT {
    pub fn new(num_arms: usize) -> UCBT {
        UCBT {
            arms: vec![
                ArmStats {
                    mean: 0.0,
                    sum_of_squares: 0.0,
                    s: 0,
                };
                num_arms
            ],
        }
    }

    pub fn std_dev(&self, j: usize) -> f64 {
        if self.arms[j].s < 2 {
            return f64::INFINITY;
        }

        let mean = self.arms[j].mean;
        let sum_of_squares = self.arms[j].sum_of_squares;
        let s = self.arms[j].s as f64;

        ((sum_of_squares / s) - mean.powi(2)).sqrt()
    }
}

impl Bandit for UCBT {
    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        let r = if reward { 1.0 } else { 0.0 };
        let prev_s = self.arms[arm].s as f64;

        self.arms[arm].s += 1;

        self.arms[arm].mean = (self.arms[arm].mean * prev_s + r) / (prev_s + 1.0);
        self.arms[arm].sum_of_squares += if reward { 1.0 } else { 0.0 };
    }

    fn pull(&mut self, _rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                if self.arms[*i].s < 2 {
                    return OrderedFloat(f64::INFINITY);
                }

                let mean = self.arms[*i].mean;
                let std_dev = self.std_dev(*i);
                let s = self.arms[*i].s;

                let v = mean + 2.326 * std_dev / (s as f64).sqrt();

                OrderedFloat(v)
            })
            .unwrap()
    }
}
