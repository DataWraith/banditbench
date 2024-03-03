use ordered_float::OrderedFloat;
use rand::prelude::*;

pub mod bandit_list;
pub mod bandits;
pub mod evaluate;

use bandits::Bandit;

pub struct BanditEvaluation {
    total_regret: f64,
    optimal_plays: usize,
}

pub fn evalute_bandit(mut b: impl Bandit, arms: &[f64], horizon: usize) -> BanditEvaluation {
    let mut reward_rngs: Vec<StdRng> = (1..=arms.len())
        .map(|i| SeedableRng::seed_from_u64(i as u64))
        .collect();

    let mut mab_rng: StdRng = SeedableRng::seed_from_u64(1);

    let mut optimal_plays = 0;
    let mut total_regret = 0.0;

    let best_arm = arms.iter().max_by_key(|&x| OrderedFloat(*x)).unwrap();

    for _ in 0..horizon {
        let arm = b.pull(&mut mab_rng);
        let reward = reward_rngs[arm].gen_bool(arms[arm]);
        b.update(arm, reward, &mut mab_rng);

        total_regret += best_arm - arms[arm];

        if arms[arm] == *best_arm {
            optimal_plays += 1;
        }
    }

    BanditEvaluation {
        total_regret,
        optimal_plays,
    }
}
