use ordered_float::OrderedFloat;
use rand::prelude::*;

use super::Arm;
use super::Bandit;

pub struct RS {
    aspiration: f64,
    arms: Vec<Arm>,
    t: usize,
}

impl RS {
    pub fn new(num_arms: usize, aleph: f64) -> Self {
        RS {
            aspiration: aleph,
            arms: vec![Arm::default(); num_arms],
            t: 1,
        }
    }
}

impl Bandit for RS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                let gain = self.arms[*i].mean() - self.aspiration;
                let n_i = self.arms[*i].n() as f64;
                let tau_i = n_i / self.t as f64;

                (OrderedFloat(tau_i * gain), rng.gen::<u32>())
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }

        self.t += 1;
    }
}
