use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::Bandit;

pub struct MARS {
    delta: f64,
    m: usize,
    t: usize,
    num_pulls: Vec<usize>,
    histories: Vec<Vec<usize>>,
    max_rewards: Vec<f64>,
    means: Vec<Vec<f64>>,
}

impl MARS {
    pub fn new(num_arms: usize, delta: f64) -> MARS {
        let m = (1.0 / delta).ceil() as usize;

        MARS {
            t: 0,
            m,
            num_pulls: vec![0; num_arms],
            histories: vec![vec![0; m]; num_arms],
            means: vec![vec![0.0; m]; num_arms],
            max_rewards: vec![0.0; num_arms],
            delta,
        }
    }

    fn ucb(&self, arm: usize, mut rng: impl Rng) -> f64 {
        if self.num_pulls[arm] == 0 {
            return f64::INFINITY;
        }

        let frac = (1.0 / self.delta).ln() / 2f64.ln();

        if (self.num_pulls[arm] as f64) < frac {
            let x: f64 = rng.gen();

            if x < self.delta * 2f64.powi(self.num_pulls[arm] as i32) {
                return f64::INFINITY;
            }

            return self.max_rewards[arm];
        }

        return self.means[arm].iter().cloned().fold(0.0, f64::max);
    }
}

impl Bandit for MARS {
    fn update(&mut self, arm: usize, reward: bool, mut rng: impl Rng) {
        let r = if reward { 1 } else { 0 };

        self.num_pulls[arm] += 1;
        self.max_rewards[arm] = self.max_rewards[arm].max(r as f64);

        for j in 0..self.m {
            let h_ji = rng.gen::<bool>();

            if h_ji {
                self.histories[arm][j] += 1;
                self.means[arm][j] = {
                    let hist_prev = self.histories[arm][j] - 1;
                    let mean_prev = self.means[arm][j];

                    (hist_prev as f64 * mean_prev + r as f64) / self.histories[arm][j] as f64
                }
            }
        }

        self.t += 1;
    }

    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.num_pulls.len())
            .map(|arm| (self.ucb(arm, &mut rng), arm))
            .max_by_key(|x| OrderedFloat(x.0))
            .unwrap()
            .1
    }
}
