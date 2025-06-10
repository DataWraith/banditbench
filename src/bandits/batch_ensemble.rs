use ordered_float::OrderedFloat;
use rand::prelude::*;

use super::Bandit;

#[derive(Clone, Default)]
struct Batch {
    loss_sum: usize,
    batch_size: usize,
}

impl Batch {
    fn update(&mut self, reward: bool) {
        if !reward {
            self.loss_sum += 1;
        }

        self.batch_size += 1;
    }

    fn mu(&self) -> f64 {
        (self.loss_sum as f64) / ((self.batch_size + 2) as f64)
    }
}

type Arm = Vec<Batch>;

pub struct BatchEnsemble {
    arms: Vec<Arm>,
    batch_size_multiplier: f64,
    t: usize,
}

impl BatchEnsemble {
    pub fn new(num_arms: usize, batch_size_multiplier: f64) -> Self {
        BatchEnsemble {
            arms: vec![vec![Batch::default()]; num_arms],
            t: 1,
            batch_size_multiplier,
        }
    }
}

impl Bandit for BatchEnsemble {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        (0..self.arms.len())
            .min_by_key(|i| {
                let batches = &self.arms[*i];

                (
                    batches.iter().map(|b| OrderedFloat(b.mu())).min().unwrap(),
                    rng.gen::<u32>(),
                )
            })
            .unwrap()
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        // The paper recommends a number of batches that is 8 * log(t), but that doesn't seem to work well.
        // Using a multiplier of 0 (i.e. always 1 batch) seems to work better, as does a multiplier of 1.
        let num_batches = (self.batch_size_multiplier * (self.t as f64).log10()) as usize;
        let num_batches = 1.max(num_batches);

        self.t += 1;

        // Add batches until we have enough
        while num_batches > self.arms[arm].len() {
            self.arms[arm].push(Batch::default());
        }

        // Evenly distribute the reward across the batches
        for i in 0..(self.arms[arm].len() - 1) {
            let a = &self.arms[arm][i];
            let b = &self.arms[arm][i + 1];

            if a.batch_size > b.batch_size {
                self.arms[arm][i + 1].update(reward);
                return;
            }
        }

        // All batches are filled equally, so we update the first one.
        self.arms[arm][0].update(reward);
    }
}
