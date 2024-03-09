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

    pub fn npts_index(&self, arm: usize, mut rng: impl Rng) -> f64 {
        let dirichlet =
            Dirichlet::new_with_size(1.0, self.arms[arm].successes + self.arms[arm].failures)
                .unwrap();

        let sample = dirichlet.sample(&mut rng);

        let mut sum = 0.0;

        for i in 0..self.arms[arm].successes {
            sum += sample[i];
        }

        sum
    }
}

impl Bandit for NPTS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| OrderedFloat(self.npts_index(*i, &mut rng)))
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
