use rand::prelude::*;

use super::Arm;
use super::Bandit;

pub const BETA: f64 = 0.5;

fn bernoulli_kl_div(p: f64, q: f64) -> f64 {
    p * (p / q).ln() + (1.0 - p) * ((1.0 - p) / (1.0 - q)).ln()
}

fn transport_cost(
    s_leader: usize,
    f_leader: usize,
    s_challenger: usize,
    f_challenger: usize,
) -> f64 {
    let leader_n = (s_leader + f_leader) as f64;
    let challenger_n = (s_challenger + f_challenger) as f64;

    let mu_i = s_leader as f64 / (s_leader + f_leader).max(1) as f64;
    let mu_j = s_challenger as f64 / (s_challenger + f_challenger).max(1) as f64;

    if mu_j >= mu_i {
        return 0.0;
    }

    let mu_ij = ((leader_n * mu_i) + (challenger_n * mu_j)) / (leader_n + challenger_n);

    leader_n * bernoulli_kl_div(mu_i, mu_ij) + challenger_n * bernoulli_kl_div(mu_j, mu_ij)
}

pub struct EBTCI {
    arms: Vec<Arm>,
    t: usize,
}

impl EBTCI {
    pub fn new(num_arms: usize) -> Self {
        EBTCI {
            t: 0,
            arms: vec![Arm::default(); num_arms],
        }
    }
}

impl EBTCI {
    pub fn empirical_best(&self, mut rng: impl Rng) -> usize {
        let mut best_arm = 0;
        let mut best_score = (
            self.arms[0].successes as f64
                / (self.arms[0].successes + self.arms[0].failures).max(1) as f64,
            rng.gen::<u32>(),
        );

        for (i, arm) in self.arms.iter().enumerate().skip(1) {
            let score = (
                arm.successes as f64 / (arm.successes + arm.failures).max(1) as f64,
                rng.gen::<u32>(),
            );

            if score > best_score {
                best_arm = i;
                best_score = score;
            }
        }

        best_arm
    }

    fn challenger(&self, leader: usize, mut rng: impl Rng) -> usize {
        let mut best_arm = 0;
        let mut best_distance = (f64::INFINITY, rng.gen::<u32>());

        for (i, challenger) in self.arms.iter().enumerate() {
            if i == leader {
                continue;
            }

            let cost_tci = transport_cost(
                self.arms[leader].successes,
                self.arms[leader].failures,
                challenger.successes,
                challenger.failures,
            );

            let distance = (
                cost_tci + ((1 + challenger.successes + challenger.failures) as f64).ln(),
                rng.gen::<u32>(),
            );

            if distance < best_distance {
                best_arm = i;
                best_distance = distance;
            }
        }

        best_arm
    }
}

impl Bandit for EBTCI {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        if self.t < self.arms.len() {
            return self.t;
        }

        let leader = self.empirical_best(&mut rng);

        if rng.gen_bool(BETA) {
            return leader;
        }

        self.challenger(leader, &mut rng)
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
