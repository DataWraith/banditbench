use ordered_float::OrderedFloat;
use rand::prelude::*;

use crate::bandits::Arm;
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

impl Bandit for BTS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms[0].len())
            .max_by_key(|i| {
                let replicate = self.arms.choose(&mut rng).unwrap();
                (OrderedFloat(replicate[*i].mean()), rng.gen::<u32>())
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, mut rng: impl Rng) {
        for j in 0..self.arms.len() {
            if rng.gen_bool(0.5) {
                continue;
            }

            if reward {
                self.arms[j][arm].successes += 1;
            } else {
                self.arms[j][arm].failures += 1;
            }
        }
    }
}
