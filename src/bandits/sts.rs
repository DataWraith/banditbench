use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Beta;

use super::Arm;
use super::Bandit;

pub struct STS {
    arms: Vec<Arm>,
    first_pull: Vec<usize>,
    t: usize,
    epsilon: f64,
}

impl STS {
    pub fn new(num_arms: usize, epsilon: f64) -> Self {
        STS {
            arms: vec![Arm::default(); num_arms],
            first_pull: vec![usize::MAX; num_arms],
            t: 0,
            epsilon,
        }
    }
}

impl Bandit for STS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        // Sample from the Beta distributions of each arm, as in TS
        let samples = (0..self.arms.len())
            .map(|i| {
                let beta = Beta::new(
                    self.arms[i].successes as f64 + 1.0,
                    self.arms[i].failures as f64 + 1.0,
                )
                .unwrap();

                beta.sample(&mut rng)
            })
            .collect::<Vec<f64>>();

        // Select the arm with the highest sample as the leader
        let leader = samples
            .iter()
            .enumerate()
            .max_by_key(|(_, sample)| OrderedFloat(**sample))
            .unwrap()
            .0;

        // Find all other arms that (a) have been pulled already and (b) have a
        // sample within epsilon of the leader
        let challengers = samples
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != leader)
            .filter(|(i, _)| self.first_pull[*i] != usize::MAX)
            .filter(|(_, s)| **s + self.epsilon >= samples[leader])
            .map(|(i, s)| (i, *s));

        // If there are any such arms, select the one with the earliest first
        // pull; otherwise, select the leader
        challengers
            .min_by_key(|(i, _)| self.first_pull[*i])
            .map(|(i, _)| i)
            .unwrap_or(leader)
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }

        self.first_pull[arm] = self.t.min(self.first_pull[arm]);
        self.t += 1;
    }
}
