use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Binomial;

use crate::bandits::Arm;
use crate::Bandit;

pub struct GIRO {
    t: usize,
    arms: Vec<Arm>,
    arms_ceil: Vec<Arm>,
    arms_floor: Vec<Arm>,
    num_pseudo_rewards: f64,
}

impl GIRO {
    pub fn new(num_arms: usize, num_pseudo_rewards: f64) -> Self {
        GIRO {
            t: 0,
            arms: vec![Arm::default(); num_arms],
            arms_ceil: vec![Arm::default(); num_arms],
            arms_floor: vec![Arm::default(); num_arms],
            num_pseudo_rewards,
        }
    }
}

impl Bandit for GIRO {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms_ceil.len() {
            return self.t;
        }

        (0..self.arms_ceil.len())
            .max_by_key(|i| {
                let s = self.arms[*i].n() as f64;
                let a = self.num_pseudo_rewards;
                let sa = a * s;
                let p_ceil = sa - sa.floor();
                let use_ceil = rng.gen_bool(p_ceil);

                let successes = if use_ceil {
                    self.arms[*i].successes + self.arms_ceil[*i].successes
                } else {
                    self.arms[*i].successes + self.arms_floor[*i].successes
                };

                let failures = if use_ceil {
                    self.arms[*i].failures + self.arms_ceil[*i].failures
                } else {
                    self.arms[*i].failures + self.arms_floor[*i].failures
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

        let s = self.arms[arm].n() as f64;
        let a = self.num_pseudo_rewards;

        while self.arms_floor[arm].n() < 2 * (s * a).floor() as usize {
            self.arms_floor[arm].successes += 1;
            self.arms_floor[arm].failures += 1;
        }

        while self.arms_ceil[arm].n() < 2 * (s * a).ceil() as usize {
            self.arms_ceil[arm].successes += 1;
            self.arms_ceil[arm].failures += 1;
        }

        self.t += 1;
    }
}
