use rand::prelude::*;

use crate::bandits::Arm;
use crate::utils::tie_break;
use crate::Bandit;

pub struct BTS {
    arms: Vec<Vec<Arm>>,
}

impl BTS {
    pub fn new(num_arms: usize, num_replicates: usize) -> BTS {
        BTS {
            arms: vec![
                vec![
                    Arm {
                        successes: 1,
                        failures: 1
                    };
                    num_arms
                ];
                num_replicates
            ],
        }
    }
}

impl std::fmt::Display for BTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bootstrapped Thompson Sampling (J={})", self.arms.len())
    }
}

impl Bandit for BTS {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        (0..self.arms[0].len())
            .max_by_key(|i| {
                let replicate = self.arms.choose(rng).unwrap();
                tie_break(replicate[*i].mean(), rng.gen::<u32>())
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, rng: &mut impl Rng) {
        for j in 0..self.arms.len() {
            if rng.gen_bool(0.5) {
                continue;
            }

            self.arms[j][arm].update(reward);
        }
    }
}
