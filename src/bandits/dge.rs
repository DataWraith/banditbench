use rand::prelude::*;
use rand_distr::WeightedIndex;
use statrs::function::beta::beta_reg;

use super::Arm;
use super::Bandit;

fn expected_improvement(arm: &Arm, v: f64) -> f64 {
    assert!(v > 0.0 && v < 1.0);

    let alpha = arm.successes as f64;
    let beta = arm.failures as f64;

    let tail_prob = 1.0 - beta_reg(alpha, beta, v);
    let tail_mean = alpha / (alpha + beta) * (1.0 - beta_reg(alpha + 1.0, beta, v));

    (tail_mean - v * tail_prob).max(0.0)
}

pub struct DGE {
    m: f64,
    gate_price: f64,
    surprisal_cap: f64,
    t: usize,
    arms: Vec<Arm>,
}

impl Default for DGE {
    fn default() -> Self {
        Self {
            m: 100.0,
            gate_price: 0.1,
            surprisal_cap: 10.0,
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
        let means = self.arms.iter().map(|a| a.mean());

        let (greedy, v) = means
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap();

        let exploit = rng.gen_bool(1.0 - self.epsilon());

        if exploit {
            return greedy;
        }

        let mut gated: Vec<f64> = (0..self.arms.len())
            .map(|i| {
                if i == greedy {
                    0.0
                } else {
                    let delight = self.surprisal_cap * expected_improvement(&self.arms[i], v);

                    if delight >= self.gate_price {
                        delight
                    } else {
                        0.0
                    }
                }
            })
            .collect();

        let sum: f64 = gated.iter().sum();

        if sum == 0.0 {
            return greedy;
        }

        gated.iter_mut().for_each(|g| *g /= sum);

        let dist = WeightedIndex::new(&gated).unwrap();
        return dist.sample(rng);
    }
}
