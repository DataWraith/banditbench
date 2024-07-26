use indicatif::{ProgressBar, ProgressStyle};
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::time::Instant;

use crate::BanditEvaluation;

use super::bandits::*;
use super::evaluate_bandit;

pub fn evaluate_bandits(
    algorithm: &Algorithms,
    num_runs: usize,
    arms_fn: impl Fn(u64) -> Vec<f64>,
    seed: u64,
    horizon: usize,
) {
    let mut results = vec![];

    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:80} {pos:>6}/{len:6} [ETA: {eta_precise}] {msg}",
    )
    .unwrap();

    let pbar = ProgressBar::new(num_runs as u64)
        .with_style(style.clone())
        .with_message(format!("{}", algorithm));

    let start = Instant::now();
    let instances = (seed..(seed + num_runs as u64))
        .map(|seed| arms_fn(seed))
        .collect::<Vec<Vec<f64>>>();

    let mut evaluations = instances
        .into_par_iter()
        .enumerate()
        .map(|(i, arms)| {
            pbar.inc(1);

            let num_arms = arms.len();
            let seed = (i + 1) as u64;

            match algorithm {
                Algorithms::BDS => {
                    let bandit = BDS::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::BGE => {
                    let bandit = BGE::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::BTS { replicates } => {
                    let bandit = BTS::new(num_arms, *replicates);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::CODE { delta } => {
                    let bandit = CODE::new(num_arms, *delta);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::EBTCI => {
                    let bandit = EBTCI::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::EpsilonDecreasing { epsilon } => {
                    let bandit = EpsilonDecreasing::new(num_arms, *epsilon);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::EpsilonGreedy { epsilon } => {
                    let bandit = EpsilonGreedy::new(num_arms, *epsilon);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::EpsTS => {
                    let bandit = EpsTS::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::EpsTSUCB { samples } => {
                    let bandit = EpsTSUCB::new(num_arms, *samples);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::ETC { m } => {
                    let bandit = ETC::new(num_arms, *m);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::EXPIX => {
                    let bandit = EXPIX::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::ForcedExploration => {
                    let bandit = ForcedExploration::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::GIRO { num_pseudo_rewards } => {
                    let bandit = GIRO::new(num_arms, *num_pseudo_rewards);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::Gradient => {
                    let bandit = GradientBandit::new(num_arms, 0.1, false);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::GradientBaseline => {
                    let bandit = GradientBandit::new(num_arms, 0.1, true);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::Greedy => {
                    let bandit = Greedy::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::HellingerUCB => {
                    let bandit = HellingerUCB::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::KLMS => {
                    let bandit = KLMS::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::KLUCB => {
                    let bandit = KLUCB::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::LilUCB { delta } => {
                    let bandit = LilUCB::new(num_arms, *delta);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::MBE => {
                    let bandit = Mbe::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::MOSSAnytime { alpha } => {
                    let bandit = MOSSAnytime::new(num_arms, *alpha);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::NPTS => {
                    let bandit = NPTS::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::OptimisticTS => {
                    let bandit = OptimisticTS::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::POKER { assumed_horizon } => {
                    let bandit = POKER::new(num_arms, *assumed_horizon);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::PFLA { n } => {
                    let bandit = PFLA::new(num_arms, *n);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::PHE { perturbation_scale } => {
                    let bandit = PHE::new(num_arms, *perturbation_scale);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::Random => {
                    let bandit = Random::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::ReBoot { r } => {
                    let bandit = ReBoot::new(num_arms, *r);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::STS { epsilon } => {
                    let bandit = STS::new(num_arms, *epsilon);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::TS => {
                    let bandit = TS::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::TSVHA => {
                    let bandit = TSVHA::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::TSUCB { samples } => {
                    let bandit = TSUCB::new(num_arms, *samples);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::TsallisINF => {
                    let bandit = TsallisINF::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::UCB1 => {
                    let bandit = UCB1::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::UCB1Tuned => {
                    let bandit = UCB1Tuned::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::UCBDT { gamma } => {
                    let bandit = UCBDT::new(num_arms, *gamma);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::UCBT => {
                    let bandit = UCBT::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::VResBoot { init } => {
                    let bandit = VResBoot::new(num_arms, *init);
                    evaluate_bandit(bandit, &arms, horizon, seed)
                }

                Algorithms::WRSDA => {
                    let bandit = WRSDA::new(num_arms);
                    evaluate_bandit(bandit, &arms, horizon, seed)
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
    // TODO: Make this into a function
    evaluations.sort_by_key(|x| OrderedFloat(x.total_regret));
    let median = evaluations[evaluations.len() / 2].total_regret;
    let mut median_deviation: Vec<f64> = evaluations
        .iter()
        .map(|x| (x.total_regret - median).abs())
        .collect();
    median_deviation.sort_by_key(|&x| OrderedFloat(x));
    let mad = median_deviation[median_deviation.len() / 2];

    results.push((algorithm, percent_optimal, mean_regret, mad, elapsed));

    results.sort_by_key(|&(_, _, mean_regret, _, _)| OrderedFloat(mean_regret));

    for (name, percent_optimal, mean_regret, mad, elapsed) in results.iter() {
        println!("{name};{percent_optimal:0.2};{mean_regret:0.4};{mad:0.4};{elapsed:0.2}s");
    }
}
