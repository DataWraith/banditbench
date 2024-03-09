use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Binomial;

use crate::bandits::Arm;
use crate::Bandit;

pub struct GIRO {
    arms: Vec<Arm>,
    t: usize,
    num_pseudo_rewards: f64,
}

impl GIRO {
    pub fn new(num_arms: usize, num_pseudo_rewards: f64) -> Self {
        GIRO {
            t: 0,
            arms: vec![Arm::default(); num_arms],
            num_pseudo_rewards,
        }
    }
}

impl Bandit for GIRO {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        (0..self.arms.len())
            .max_by_key(|i| {
                let a = self.num_pseudo_rewards;
                let s = self.arms[*i].n() as f64;

                let ceil_prob = (a * s) - (a * s).floor();
                let use_ceil = rng.gen_bool(ceil_prob);

                let successes = if use_ceil {
                    self.arms[*i].successes + (s * a).ceil() as usize
                } else {
                    self.arms[*i].successes + (s * a).floor() as usize
                };

                let failures = if use_ceil {
                    self.arms[*i].failures + (s * a).ceil() as usize
                } else {
                    self.arms[*i].failures + (s * a).floor() as usize
                };

                if successes == 0 {
                    return (OrderedFloat(0.0), rng.gen::<u32>());
                }

                let d = Binomial::new(
                    (successes + failures) as u64,
                    successes as f64 / (successes + failures) as f64,
                )
                .unwrap();

                let mu = d.sample(&mut rng) as f64 / (successes + failures) as f64;

                (OrderedFloat(mu), rng.gen::<u32>())
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
