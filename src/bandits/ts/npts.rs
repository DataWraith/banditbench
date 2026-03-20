use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Dirichlet;

use crate::bandits::Arm;
use crate::Bandit;

pub struct NPTS {
    arms: Vec<Arm>,
    t: usize,
}

impl NPTS {
    pub fn new(num_arms: usize) -> Self {
        NPTS {
            t: 0,
            arms: vec![
                Arm {
                    successes: 1, // Exploration bonus
                    failures: 0
                };
                num_arms
            ],
        }
    }

    pub fn npts_index(&self, arm: usize, rng: &mut impl Rng) -> f64 {
        let dirichlet = Dirichlet::new_with_size(1.0, self.arms[arm].n()).unwrap();

        let sample = dirichlet.sample(rng);

        let mut sum = 0.0;

        for i in 0..self.arms[arm].successes {
            sum += sample[i];
        }

        sum
    }
}

impl std::fmt::Display for NPTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Non-Parametric Thompson Sampling")
    }
}

impl Bandit for NPTS {
    fn pull(&mut self, rng: &mut impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| OrderedFloat(self.npts_index(*i, rng)))
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: &mut impl Rng) {
        self.arms[arm].update(reward);
        self.t += 1;
    }
}
