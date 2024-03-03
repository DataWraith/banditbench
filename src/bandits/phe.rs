use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Binomial;

use super::Arm;
use super::Bandit;

pub struct PHE {
    arms: Vec<Arm>,
    t: usize,
    perturbation_scale: f64,
}

impl PHE {
    pub fn new(n: usize, perturbation_scale: f64) -> Self {
        Self {
            t: 0,
            arms: vec![Arm::default(); n],
            perturbation_scale,
        }
    }
}

impl Bandit for PHE {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                let s = self.arms[*i].n() as f64;
                let a = self.perturbation_scale;
                let v = self.arms[*i].successes as f64;
                let d = Binomial::new((a * s).ceil() as u64, 1.0 / 2.0).unwrap();
                let u = d.sample(&mut rng) as f64;
                let mu = (v + u) / (s + (a * s).ceil());

                OrderedFloat(mu)
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
