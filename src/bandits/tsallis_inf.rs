use rand::prelude::*;

use super::Bandit;

pub struct TsallisINF {
    w: Vec<f64>,
    x: f64,
    loss: Vec<f64>,
    t: usize,
}

impl TsallisINF {
    pub fn new(num_arms: usize) -> Self {
        let mut inf = TsallisINF {
            w: vec![1.0; num_arms],
            x: 1.0,
            loss: vec![0.0; num_arms],
            t: 1,
        };

        inf.tsallis_inf();
        inf
    }

    pub fn estimate_loss(&mut self, arm: usize, reward: bool) -> Vec<f64> {
        let lr = 4.0 * (1.0 / self.t as f64).sqrt();
        let loss = if reward { 0.0 } else { 1.0 };

        let mut l = vec![0.0; self.w.len()];

        for i in 0..self.w.len() {
            let mut b = 0.0;

            if self.w[i] >= lr * lr {
                b = 0.5;
            }

            if i == arm {
                l[i] = (loss - b) / self.w[i];
            }

            l[i] += b;
        }

        l
    }

    pub fn tsallis_inf(&mut self) {
        let mut prev_w = self.w.clone();
        let lr = 4.0 * (1.0 / self.t as f64).sqrt();

        for _ in 0..20 {
            for i in 0..self.w.len() {
                self.w[i] = 4.0 * (lr * (self.loss[i] - self.x)).powi(-2);
            }

            self.x -= (self.w.iter().sum::<f64>() - 1.0)
                / (lr * self.w.iter().map(|w| w.powf(3.0 / 2.0)).sum::<f64>());

            if self
                .w
                .iter()
                .zip(prev_w.iter())
                .all(|(a, b)| (a - b).abs() < 1e-6)
            {
                break;
            }

            prev_w = self.w.clone()
        }
    }
}

impl Bandit for TsallisINF {
    fn pull(&mut self, mut rng: impl Rng) -> usize {
        let w_sum = self.w.iter().sum::<f64>();
        let mut choice = rng.gen_range(0.0..w_sum);

        for arm in 0..self.w.len() {
            choice -= self.w[arm];

            if choice <= 0.0 {
                return arm;
            }
        }

        return self.w.len() - 1;
    }

    fn update(&mut self, arm: usize, reward: bool, _rng: impl Rng) {
        let loss_vec = self.estimate_loss(arm, reward);

        self.loss = self
            .loss
            .iter()
            .zip(loss_vec.iter())
            .map(|(a, b)| a + b)
            .collect();

        self.t += 1;

        self.tsallis_inf();
    }
}
