use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Gumbel;

use super::Arm;
use super::Bandit;

pub struct SoftSatisficing {
    aspiration: f64,
    arms: Vec<Arm>,
}

impl SoftSatisficing {
    pub fn new(num_arms: usize, aleph: f64) -> Self {
        SoftSatisficing {
            aspiration: aleph,
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl Bandit for SoftSatisficing {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        let gumbel = Gumbel::new(0.0, 1.0).unwrap();

        (0..self.arms.len())
            .filter(|&i| self.arms[i].mean() >= self.aspiration)
            .choose(&mut rng)
            .unwrap_or_else(|| {
                (0..self.arms.len())
                    .max_by_key(|&i| {
                        let e_rs = -(self.aspiration - self.arms[i].mean()).ln();
                        let gumbel_sample = gumbel.sample(&mut rng);

                        OrderedFloat(e_rs + gumbel_sample)
                    })
                    .unwrap()
            })
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }
    }
}
