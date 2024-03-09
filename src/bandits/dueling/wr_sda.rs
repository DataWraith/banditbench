use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Hypergeometric;

use crate::bandits::Arm;
use crate::Bandit;

pub struct WRSDA {
    arms: Vec<Arm>,
    challengers: Vec<usize>,
}

impl WRSDA {
    pub fn new(num_arms: usize) -> Self {
        WRSDA {
            arms: vec![Arm::default(); num_arms],
            challengers: (0..num_arms).collect(),
        }
    }

    pub fn leader(&self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                (
                    self.arms[*i].n(),
                    OrderedFloat(self.arms[*i].mean()),
                    rng.gen::<u32>(),
                )
            })
            .unwrap()
    }

    pub fn duel(&self, leader: usize, challenger: usize, mut rng: impl Rng) -> bool {
        // Determine Challenger score
        let challenger_score = self.arms[challenger].mean();
        let challenger_n = self.arms[challenger].n();

        // Determine Leader score
        let n = self.arms[leader].n() as u64;
        let k = self.arms[leader].successes as u64;
        let x = challenger_n as u64;

        let dist = Hypergeometric::new(n, k, x).unwrap();
        let leader_score = dist.sample(&mut rng) as f64 / challenger_n as f64;

        // Return the result of the duel
        challenger_score > leader_score
    }
}

impl Bandit for WRSDA {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if !self.challengers.is_empty() {
            return self.challengers.pop().unwrap();
        }

        let leader = self.leader(&mut rng);

        for i in 0..self.arms.len() {
            if i == leader {
                continue;
            }

            if self.duel(leader, i, &mut rng) {
                self.challengers.push(i);
            }
        }

        if self.challengers.is_empty() {
            return leader;
        }

        self.challengers.shuffle(&mut rng);
        self.challengers.pop().unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }
    }
}
