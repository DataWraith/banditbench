use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
use crate::Bandit;

pub const EPSILON: f64 = 0.0;
pub const BETA: f64 = 0.5;

pub struct LilUCB {
    arms: Vec<Arm>,
    t: usize,
    delta: f64,
}

impl LilUCB {
    pub fn new(num_arms: usize, delta: f64) -> LilUCB {
        LilUCB {
            t: 0,
            arms: vec![Arm::default(); num_arms],
            delta,
        }
    }
}

impl std::fmt::Display for LilUCB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lil' UCB (δ={:.3})", self.delta)
    }
}

impl Bandit for LilUCB {
    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        self.arms[arm].update(reward);

        self.t += 1;
    }

    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                let n_j = self.arms[*i].n() as f64;
                let sigma2 = 0.25;

                let cb = (1.0 + BETA) * (1.0 + EPSILON.sqrt());
                let cb = cb
                    * ((2.0
                        * sigma2
                        * (1.0 + EPSILON)
                        * (((1.0 + EPSILON) * n_j).ln() / self.delta).ln())
                        / n_j)
                        .sqrt();

                (OrderedFloat(self.arms[*i].mean() + cb), rng.gen::<u32>())
            })
            .unwrap()
    }
}
