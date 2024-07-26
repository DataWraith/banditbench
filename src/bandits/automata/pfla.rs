use rand::prelude::*;
use rand_distr::Beta;

use crate::bandits::Arm;
use crate::Bandit;

pub struct PFLA {
    arms: Vec<Arm>,
    n: usize,
}

impl PFLA {
    pub fn new(num_arms: usize, n: usize) -> Self {
        PFLA {
            arms: vec![Arm::default(); num_arms],
            n,
        }
    }
}

impl Bandit for PFLA {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        let mut wins = vec![0; self.arms.len()];

        for _ in 0..self.n {
            let mut winner = 0;
            let mut best = f64::NEG_INFINITY;

            for (j, arm) in self.arms.iter().enumerate() {
                let beta =
                    Beta::new(arm.successes as f64 + 2.0, arm.failures as f64 + 1.0).unwrap();

                let sample = beta.sample(&mut rng);

                if sample > best {
                    best = sample;
                    winner = j;
                }
            }

            wins[winner] += 1;
        }

        let mut estimates = (0..self.arms.len())
            .map(|i| (wins[i], rng.gen::<u32>(), i))
            .collect::<Vec<_>>();

        estimates.sort();

        let (_wins, _tiebreaker, a1) = estimates.pop().unwrap();
        let (_wins, _tiebreaker, a2) = estimates.pop().unwrap();

        let arm1 = &self.arms[a1];
        let arm2 = &self.arms[a2];

        if arm1.n() < arm2.n() {
            return a1;
        }

        if arm2.n() < arm1.n() {
            return a2;
        }

        if rng.gen() {
            return a1;
        }

        return a2;
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }
    }
}
