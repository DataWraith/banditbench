use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::Bandit;

#[derive(Clone)]
pub struct ArmStats {
    mean: f64,
    sum_of_squares: f64,
    s: usize,
}

pub struct UCB1Tuned {
    arms: Vec<ArmStats>,
    t: usize,
}

impl UCB1Tuned {
    pub fn new(num_arms: usize) -> UCB1Tuned {
        UCB1Tuned {
            t: 0,
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

    pub fn var_j(&self, j: usize) -> f64 {
        if self.arms[j].s == 0 {
            return f64::INFINITY;
        }

        let mean = self.arms[j].mean;
        let sum_of_squares = self.arms[j].sum_of_squares;
        let s = self.arms[j].s as f64;
        let t = self.t as f64;

        ((1.0 / s) * sum_of_squares) - mean.powi(2) + (2.0 * t.ln() / s).sqrt()
    }
}

impl Bandit for UCB1Tuned {
    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        let r = if reward { 1.0 } else { 0.0 };
        let prev_s = self.arms[arm].s as f64;

        self.t += 1;
        self.arms[arm].s += 1;

        self.arms[arm].mean = (self.arms[arm].mean * prev_s + r) / (prev_s + 1.0);
        self.arms[arm].sum_of_squares += if reward { 1.0 } else { 0.0 };
    }

    fn pull(&mut self, _rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                if self.arms[*i].s == 0 {
                    return OrderedFloat(f64::INFINITY);
                }

                let mean = self.arms[*i].mean;
                let var_j = self.var_j(*i);
                let t = self.t as f64;

                let ucb = mean + ((t.ln() / self.arms[*i].s as f64) * 0.25f64.min(var_j)).sqrt();

                OrderedFloat(ucb)
            })
            .unwrap()
    }
}
