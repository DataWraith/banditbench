use rand::prelude::*;
use rand_distr::Gumbel;

use crate::utils::tie_break;

use super::Arm;
use super::Bandit;

pub struct BGE {
    arms: Vec<Arm>,
}

impl BGE {
    pub fn new(num_arms: usize) -> Self {
        BGE {
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl std::fmt::Display for BGE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Boltzmann-Gumbel Exploration")
    }
}

impl Bandit for BGE {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        let gumbel = Gumbel::new(0.0, 1.0).unwrap();

        (0..self.arms.len())
            .max_by_key(|i| {
                let w = self.arms[*i].successes;
                let l = self.arms[*i].failures;

                if w + l == 0 {
                    tie_break(f64::INFINITY, rng.gen::<u32>())
                } else {
                    let mean = w as f64 / (w + l) as f64;
                    let beta = (0.25 / (w + l) as f64).sqrt();
                    let z = gumbel.sample(rng);

                    tie_break(mean + beta * z, rng.gen::<u32>())
                }
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
    }
}
