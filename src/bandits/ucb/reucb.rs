use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub struct ReUCB {
    arms: Vec<Arm>,
    reward_sum: f64,
    t: usize,
    cached_sigma_squared: f64,
    cached_sigma_zero_squared: f64,
    a: f64,
}

impl ReUCB {
    pub fn new(num_arms: usize, a: f64) -> Self {
        ReUCB {
            arms: vec![Arm::default(); num_arms],
            reward_sum: 0.0,
            t: 0,
            cached_sigma_squared: 0.0,
            cached_sigma_zero_squared: 0.0,
            a,
        }
    }

    fn ucb(&self, arm: usize) -> f64 {
        let mean = self.arms[arm].mean();
        let c = (self.a * self.tau_estimate(arm).powi(2) * ((1 + self.t) as f64).ln()).sqrt();

        mean + c
    }

    fn tau_estimate(&self, arm: usize) -> f64 {
        let sigma_squared = self.cached_sigma_squared;
        let sigma_zero_squared = self.cached_sigma_zero_squared;

        let w_s = (0..self.arms.len())
            .map(|i| self.w(i, sigma_squared, sigma_zero_squared))
            .collect::<Vec<_>>();

        let leading_term = w_s[arm] * (self.arms[arm].n() as f64).powi(-1) * sigma_squared;

        let numerator = (1.0 - w_s[arm]).powi(2) * sigma_squared;
        let mut denominator = 0.0;

        for i in 0..self.arms.len() {
            denominator += self.arms[i].n() as f64 * (1.0 - w_s[i]);
        }

        leading_term + numerator / denominator
    }

    fn w(&self, arm: usize, sigma_squared: f64, sigma_zero_squared: f64) -> f64 {
        1.0 / (1.0 + (self.arms[arm].n() as f64).powi(-1) * sigma_squared / sigma_zero_squared)
    }

    fn n_k_sum(&self) -> f64 {
        self.t as f64
    }

    fn n_k_m1_sum(&self) -> f64 {
        self.t as f64 - self.arms.len() as f64
    }

    fn n_k_sq_sum(&self) -> f64 {
        self.arms
            .iter()
            .map(|arm| arm.n() as f64 * arm.n() as f64)
            .sum()
    }

    fn n_star(&self, n_k_sum: f64) -> f64 {
        let n_k_sq_sum = self.n_k_sq_sum();

        n_k_sum - n_k_sum.powi(-1) * n_k_sq_sum
    }

    fn u_k(&self, arm: usize, n_k_sum: f64) -> f64 {
        let r_k = self.arms[arm].mean();

        r_k - n_k_sum.powi(-1) * self.reward_sum
    }

    fn sigma_zero_squared(&self) -> f64 {
        let n_k_sum = self.n_k_sum();
        let n_star = self.n_star(n_k_sum);

        let mut sum = 0.0;

        for arm in 0..self.arms.len() {
            let u_k = self.u_k(arm, n_k_sum);
            let n_k = self.arms[arm].n() as f64;
            sum += u_k * u_k * n_k;
        }

        sum / n_star
    }

    fn sigma_squared(&self) -> f64 {
        let n_k_m1_sum = self.n_k_m1_sum();
        let mut sum = 0.0;

        for arm in 0..self.arms.len() {
            let s = self.arms[arm].successes as f64;
            let f = self.arms[arm].failures as f64;
            let m = self.arms[arm].mean();

            sum += (1.0 - m) * (1.0 - m) * s;
            sum += (0.0 - m) * (0.0 - m) * f;
        }

        sum / n_k_m1_sum
    }
}

impl Bandit for ReUCB {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        self.cached_sigma_squared = self.sigma_squared();
        self.cached_sigma_zero_squared = self.sigma_zero_squared();

        (0..self.arms.len())
            .map(|arm| (arm, (OrderedFloat(self.ucb(arm)), rng.gen::<u32>())))
            .max_by_key(|(_, ucb)| *ucb)
            .unwrap()
            .0
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        self.reward_sum += if reward { 1.0 } else { 0.0 };
        self.t += 1;

        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }
    }
}
