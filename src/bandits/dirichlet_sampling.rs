use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Dirichlet;

use super::Arm;
use super::Bandit;

pub struct BDS {
    arms: Vec<Arm>,
    challengers: Vec<usize>,
}

impl BDS {
    pub fn new(num_arms: usize) -> Self {
        BDS {
            challengers: (0..num_arms).collect(),
            arms: vec![
                Arm {
                    successes: 1, // Exploration bonus
                    failures: 0
                };
                num_arms
            ],
        }
    }

    pub fn leader(&self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .max_by_key(|i| {
                (
                    self.arms[*i].successes + self.arms[*i].failures,
                    OrderedFloat(
                        self.arms[*i].successes as f32
                            / (self.arms[*i].successes + self.arms[*i].failures) as f32,
                    ),
                    rng.gen::<u32>(),
                )
            })
            .unwrap()
    }

    pub fn bds_index(&self, challenger: usize, mut rng: impl Rng) -> f64 {
        let dirichlet = Dirichlet::new_with_size(
            1.0,
            self.arms[challenger].successes + self.arms[challenger].failures,
        )
        .unwrap();

        let sample = dirichlet.sample(&mut rng);

        let mut sum = 0.0;

        for i in 0..self.arms[challenger].successes {
            sum += sample[i];
        }

        sum
    }
}

impl Bandit for BDS {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if !self.challengers.is_empty() {
            return self.challengers.pop().unwrap();
        }

        let leader = self.leader(&mut rng);
        let leader_len = self.arms[leader].successes + self.arms[leader].failures;
        let leader_mean = self.arms[leader].successes as f64 / leader_len as f64;

        for i in 0..self.arms.len() {
            if i == leader {
                continue;
            }

            let challenger_len = self.arms[i].successes + self.arms[i].failures;
            let challenger_mean = self.arms[i].successes as f64 / challenger_len as f64;

            if challenger_len >= leader_len {
                continue;
            }

            if leader_mean <= challenger_mean {
                self.challengers.push(i);
                continue;
            }

            let challenger_index = self.bds_index(i, &mut rng);

            if leader_mean <= challenger_index {
                self.challengers.push(i);
            }
        }

        if self.challengers.is_empty() {
            leader
        } else {
            self.challengers.shuffle(&mut rng);
            self.challengers.pop().unwrap()
        }
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        if reward {
            self.arms[arm].successes += 1;
        } else {
            self.arms[arm].failures += 1;
        }
    }
}
