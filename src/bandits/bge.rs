use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Gumbel;

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

impl Bandit for BGE {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        let gumbel = Gumbel::new(0.0, 1.0).unwrap();

        (0..self.arms.len())
            .max_by_key(|i| {
                let w = self.arms[*i].successes;
                let l = self.arms[*i].failures;

                if w + l == 0 {
                    (OrderedFloat(f64::INFINITY), rng.gen::<u32>())
                } else {
                    let mean = w as f64 / (w + l) as f64;
                    let beta = (0.25 / (w + l) as f64).sqrt();
                    let z = gumbel.sample(&mut rng);

                    (OrderedFloat(mean + beta * z), rng.gen::<u32>())
                }
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }
    }
}
