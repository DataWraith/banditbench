use rand::prelude::*;
use rand_distr::WeightedIndex;
use statrs::function::beta::beta_reg;

use crate::utils::softmax;

use super::Arm;
use super::Bandit;

fn expected_improvement(arm: &Arm, v: f64) -> f64 {
    assert!(v > 0.0 && v < 1.0);

    let alpha = arm.successes as f64;
    let beta = arm.failures as f64;

    let tail_prob = 1.0 - beta_reg(alpha, beta, v);
    let tail_mean = alpha / (alpha + beta) * (1.0 - beta_reg(alpha + 1.0, beta, v));

    tail_mean - v * tail_prob
}

pub struct DGE {
    m: f64,
    lambda: f64,
    saturation: f64,
    temp: f64,
    t: usize,
    arms: Vec<Arm>,
}

impl Default for DGE {
    fn default() -> Self {
        Self {
            m: 100.0,
            lambda: 0.1,
            saturation: 10.0,
            temp: 1.0 / 5000.0,
            t: 0,
            arms: vec![],
        }
    }
}

impl std::fmt::Display for DGE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Delight-gated Exploration")
    }
}

impl DGE {
    pub fn new(num_arms: usize) -> Self {
        Self {
            arms: vec![
                Arm {
                    successes: 1,
                    failures: 1
                };
                num_arms
            ],

            ..Default::default()
        }
    }

    fn epsilon(&self) -> f64 {
        self.m / (self.m + self.t as f64)
    }
}

impl Bandit for DGE {
    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
        self.t += 1;
    }

    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        let means: Vec<f64> = self.arms.iter().map(|a| a.mean()).collect();
        let temp_means: Vec<f64> = means.iter().map(|m| m / self.temp).collect();
        let pi_host = softmax(&temp_means);

        let exploit = rng.gen_bool(1.0 - self.epsilon());

        if exploit {
            let dist = WeightedIndex::new(&pi_host).unwrap();
            return dist.sample(rng);
        }

        let v = means.iter().max_by(|a, b| a.total_cmp(&b)).unwrap();

        let neg_log_probs: Vec<f64> = pi_host.iter().map(|prob| -prob.ln()).collect();
        let l_min = neg_log_probs.iter().min_by(|a, b| a.total_cmp(&b)).unwrap();

        let expected_improvement = self.arms.iter().map(|a| expected_improvement(a, *v));

        let ls = neg_log_probs
            .iter()
            .map(|log_prob| (log_prob - l_min).clamp(0.0, self.saturation));

        let delights = expected_improvement.zip(ls).map(|(ei, l)| ei * l);

        let mut gated: Vec<f64> = delights
            .map(|d| if d >= self.lambda { d } else { 0.0 })
            .collect();

        let sum: f64 = gated.iter().sum();
        gated.iter_mut().for_each(|g| *g /= sum);

        let policy = if sum == 0.0 { pi_host } else { gated };

        let dist = WeightedIndex::new(&policy).unwrap();
        return dist.sample(rng);
    }
}
