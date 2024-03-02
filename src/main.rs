use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use ordered_float::OrderedFloat;
use rand::prelude::*;
use rayon::prelude::*;

mod bandits;
use bandits::{
    bge::BGE, dirichlet_sampling::BDS, ebtci::EBTCI, eps_ts::EpsTS,
    gradient_bandit::GradientBandit, greedy::Greedy, kl_ucb::KLUCB, klms::KLMS, mbe::Mbe,
    npts::NPTS, phe::PHE, random::Random, ts::TS, ts_vha::TSVHA, tsucb::TSUCB, ucb1::UCB1,
    ucb1_tuned::UCB1Tuned, wr_sda::WRSDA, Algorithms, Bandit,
};

pub const SEED: u64 = 1_234_567;
pub const HORIZON: usize = 500;
pub const NUM_ARMS: usize = 10;
pub const NUM_RUNS: usize = 10001; // Odd, to simplify median computation

fn evalute_bandit(mut b: impl Bandit, arms: &[f64]) -> f64 {
    let mut reward_rngs: Vec<StdRng> = (1..=arms.len())
        .map(|i| SeedableRng::seed_from_u64(i as u64))
        .collect();

    let mut mab_rng: StdRng = SeedableRng::seed_from_u64(2);
    let mut total_regret = 0.0;

    let best_arm = arms.iter().max_by_key(|&x| OrderedFloat(*x)).unwrap();

    for _ in 0..HORIZON {
        let arm = b.pull(&mut mab_rng);
        let reward = reward_rngs[arm].gen_bool(arms[arm]);
        b.update(arm, reward, &mut mab_rng);
        total_regret += best_arm - arms[arm];
    }

    total_regret
}

fn main() {
    let mut results = vec![];

    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:80} {pos:>6}/{len:6} [ETA: {eta_precise}] {msg}",
    )
    .unwrap();

    for algorithm in [
        Algorithms::BDS,
        Algorithms::BGE,
        Algorithms::EBTCI,
        Algorithms::EpsTS,
        Algorithms::Gradient,
        Algorithms::GradientBaseline,
        Algorithms::Greedy,
        Algorithms::KLMS,
        Algorithms::KLUCB,
        Algorithms::MBE,
        Algorithms::NPTS,
        Algorithms::PHE,
        Algorithms::Random,
        Algorithms::TS,
        Algorithms::TSUCB,
        Algorithms::TSVHA,
        Algorithms::UCB1,
        Algorithms::UCB1Tuned,
        Algorithms::WRSDA,
    ] {
        let pbar = ProgressBar::new(NUM_RUNS as u64)
            .with_style(style.clone())
            .with_message(format!("{}", algorithm));

        let start = Instant::now();

        let mut regrets = (SEED..(SEED + NUM_RUNS as u64))
            .into_par_iter()
            .map(|seed| {
                pbar.inc(1);

                let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
                let arms: Vec<f64> = (0..NUM_ARMS).map(|_| rng.gen_range(0.0..=1.0)).collect();

                match algorithm {
                    Algorithms::BDS => {
                        let bandit = BDS::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::BGE => {
                        let bandit = BGE::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::EBTCI => {
                        let bandit = EBTCI::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::EpsTS => {
                        let bandit = EpsTS::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::Gradient => {
                        let bandit = GradientBandit::new(NUM_ARMS, 0.1, false);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::GradientBaseline => {
                        let bandit = GradientBandit::new(NUM_ARMS, 0.1, true);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::Greedy => {
                        let bandit = Greedy::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::KLMS => {
                        let bandit = KLMS::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::KLUCB => {
                        let bandit = KLUCB::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::MBE => {
                        let bandit = Mbe::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::NPTS => {
                        let bandit = NPTS::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::PHE => {
                        let bandit = PHE::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::Random => {
                        let bandit = Random::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::TS => {
                        let bandit = TS::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::TSVHA => {
                        let bandit = TSVHA::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::TSUCB => {
                        let bandit = TSUCB::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::UCB1 => {
                        let bandit = UCB1::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::UCB1Tuned => {
                        let bandit = UCB1Tuned::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }

                    Algorithms::WRSDA => {
                        let bandit = WRSDA::new(NUM_ARMS);
                        evalute_bandit(bandit, &arms)
                    }
                }
            })
            .collect::<Vec<f64>>();

        let elapsed = start.elapsed().as_secs_f64();

        pbar.finish();

        let mean_regret = regrets.iter().sum::<f64>() / regrets.len() as f64;

        // Quick and dirty Median Absolute Deviation computation
        regrets.sort_by_key(|&x| OrderedFloat(x));
        let median = regrets[regrets.len() / 2];
        let mut median_deviation: Vec<f64> = regrets.iter().map(|&x| (x - median).abs()).collect();
        median_deviation.sort_by_key(|&x| OrderedFloat(x));
        let mad = median_deviation[median_deviation.len() / 2];

        results.push((algorithm, mean_regret, mad, elapsed));
    }

    results.sort_by_key(|&(_, mean_regret, _, _)| OrderedFloat(mean_regret));

    println!("| Algorithm | Regret (Mean) | Regret (Median Absolute Deviation) | Time |");
    println!("|---|---:|---:|:--:|");

    for (name, mean_regret, mad, elapsed) in results.iter() {
        println!("| {name} | {mean_regret:0.4} | {mad:0.4} | {elapsed:0.4}s |",);
    }
}
