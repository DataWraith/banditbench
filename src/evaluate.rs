use indicatif::{ProgressBar, ProgressStyle};
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::time::Instant;

use crate::BanditEvaluation;

use super::bandit_list::*;
use super::evalute_bandit;

pub fn evaluate_all_bandits(num_runs: usize, arms_fn: impl Fn(u64) -> Vec<f64>, horizon: usize) {
    let mut results = vec![];

    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:80} {pos:>6}/{len:6} [ETA: {eta_precise}] {msg}",
    )
    .unwrap();

    for algorithm in ALL_BANDITS.iter() {
        let pbar = ProgressBar::new(num_runs as u64)
            .with_style(style.clone())
            .with_message(format!("{}", algorithm));

        let start = Instant::now();
        let instances = (1..=(num_runs as u64))
            .map(|seed| arms_fn(seed))
            .collect::<Vec<Vec<f64>>>();

        let mut evaluations = instances
            .into_par_iter()
            .map(|arms| {
                pbar.inc(1);

                let num_arms = arms.len();

                match algorithm {
                    Algorithms::BDS => {
                        let bandit = BDS::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::BGE => {
                        let bandit = BGE::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::EBTCI => {
                        let bandit = EBTCI::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::EpsTS => {
                        let bandit = EpsTS::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::GIRO => {
                        let bandit = GIRO::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::Gradient => {
                        let bandit = GradientBandit::new(num_arms, 0.1, false);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::GradientBaseline => {
                        let bandit = GradientBandit::new(num_arms, 0.1, true);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::Greedy => {
                        let bandit = Greedy::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::KLMS => {
                        let bandit = KLMS::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::KLUCB => {
                        let bandit = KLUCB::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::MBE => {
                        let bandit = Mbe::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::NPTS => {
                        let bandit = NPTS::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::PHE => {
                        let bandit = PHE::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::Random => {
                        let bandit = Random::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::TS => {
                        let bandit = TS::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::TSVHA => {
                        let bandit = TSVHA::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::TSUCB => {
                        let bandit = TSUCB::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::UCB1 => {
                        let bandit = UCB1::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::UCB1Tuned => {
                        let bandit = UCB1Tuned::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }

                    Algorithms::WRSDA => {
                        let bandit = WRSDA::new(num_arms);
                        evalute_bandit(bandit, &arms, horizon)
                    }
                }
            })
            .collect::<Vec<BanditEvaluation>>();

        let elapsed = start.elapsed().as_secs_f64();

        pbar.finish();

        let mean_regret = evaluations
            .iter()
            .map(|eval| eval.total_regret)
            .sum::<f64>()
            / evaluations.len() as f64;

        let percent_optimal = 100.0
            * evaluations
                .iter()
                .map(|eval| eval.optimal_plays)
                .sum::<usize>() as f64
            / (evaluations.len() * horizon) as f64;

        // Quick and dirty Median Absolute Deviation computation
        evaluations.sort_by_key(|x| OrderedFloat(x.total_regret));
        let median = evaluations[evaluations.len() / 2].total_regret;
        let mut median_deviation: Vec<f64> = evaluations
            .iter()
            .map(|x| (x.total_regret - median).abs())
            .collect();
        median_deviation.sort_by_key(|&x| OrderedFloat(x));
        let mad = median_deviation[median_deviation.len() / 2];

        results.push((algorithm, percent_optimal, mean_regret, mad, elapsed));
    }

    results.sort_by_key(|&(_, _, mean_regret, _, _)| OrderedFloat(mean_regret));

    println!(
        "| Algorithm | %-Optimal | Regret (Mean) | Regret (Mdian Absolute Deviation) | Time |"
    );
    println!("|---|---:|---:|---:|:--:|");

    for (name, percent_optimal, mean_regret, mad, elapsed) in results.iter() {
        println!(
            "| {name} | {percent_optimal:0.2}% | {mean_regret:0.4} | {mad:0.4} | {elapsed:0.2}s |",
        );
    }
}
