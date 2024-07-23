use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Gumbel;

use super::Bandit;

pub struct EXPIX {
    losses: Vec<f64>,
    t: usize,
}

impl EXPIX {
    pub fn new(num_arms: usize) -> Self {
        EXPIX {
            losses: vec![0.0; num_arms],
            t: 1,
        }
    }
}

impl EXPIX {
    pub fn learning_rate(&self) -> f64 {
        let num_arms = self.losses.len() as f64;

        (num_arms.ln() / (num_arms * self.t as f64)).sqrt()
    }
}

impl Bandit for EXPIX {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        let gumbel = Gumbel::new(0.0, 1.0).unwrap();
        let lr = self.learning_rate();

        self.losses
            .iter()
            .enumerate()
            .map(|(i, &loss)| {
                let noise = gumbel.sample(&mut rng);
                (-lr * loss + noise, i)
            })
            .max_by_key(|(index, _)| OrderedFloat(*index))
            .unwrap()
            .1
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        let reward = if reward { 1.0 } else { 0.0 };

        let learning_rate = self.learning_rate();
        let gamma = learning_rate / 2.0;

        let sampling_logits = self
            .losses
            .iter()
            .map(|loss| (-learning_rate * loss))
            .collect::<Vec<f64>>();

        let max_logit = sampling_logits
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let sampling_dist_unnormalized = sampling_logits.iter().map(|x| (x - max_logit).exp());
        let sampling_dist_sum = sampling_dist_unnormalized.clone().sum::<f64>();
        let mut sampling_dist = sampling_dist_unnormalized.map(|x| x / sampling_dist_sum);

        self.losses[arm] += (1.0 - reward) / (sampling_dist.nth(arm).unwrap() + gamma);
        self.t += 1;
    }
}
