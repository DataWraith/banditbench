use indicatif::{ProgressBar, ProgressStyle};
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::time::Instant;

use crate::BanditEvaluation;

use super::bandits::*;
use super::evaluate_bandit;

macro_rules! evaluate_match {
    ($algorithm:expr, $num_arms:expr, $arms:expr, $horizon:expr, $seed:expr, {
        $($pattern:pat => $constructor:expr),* $(,)?
    }) => {
        match $algorithm {
            $($pattern => {
                let bandit = $constructor;
                evaluate_bandit(bandit, $arms, $horizon, $seed)
            }),*
        }
    };
}

pub fn evaluate_bandits(
    algorithm: &Algorithms,
    num_runs: usize,
    arms_fn: impl Fn(u64) -> Vec<f64>,
    seed: u64,
    horizon: usize,
) {

    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:80} {pos:>6}/{len:6} [ETA: {eta_precise}] {msg}",
    )
    .unwrap();

    let pbar = ProgressBar::new(num_runs as u64)
        .with_style(style)
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

            evaluate_match!(algorithm, num_arms, &arms, horizon, seed, {
                Algorithms::BatchEnsemble { multiplier } => BatchEnsemble::new(num_arms, *multiplier),
                Algorithms::BayesUCB { delta } => BayesUCB::new(num_arms, *delta),
                Algorithms::BDS => BDS::new(num_arms),
                Algorithms::BGE => BGE::new(num_arms),
                Algorithms::BrezziLaiApprox { beta } => BrezziLaiApprox::new(num_arms, *beta),
                Algorithms::BTS { replicates } => BTS::new(num_arms, *replicates),
                Algorithms::CODE { delta } => CODE::new(num_arms, *delta),
                Algorithms::DelightfulGradient { lr } => DelightfulGradientBandit::new(num_arms, *lr),
                Algorithms::EBTCI => EBTCI::new(num_arms),
                Algorithms::EpsilonDecreasing { epsilon } => EpsilonDecreasing::new(num_arms, *epsilon),
                Algorithms::EpsilonGreedy { epsilon } => EpsilonGreedy::new(num_arms, *epsilon),
                Algorithms::EpsTS => EpsTS::new(num_arms),
                Algorithms::EpsTSUCB { samples } => EpsTSUCB::new(num_arms, *samples),
                Algorithms::ETC { m } => ETC::new(num_arms, *m),
                Algorithms::EXPIX => EXPIX::new(num_arms),
                Algorithms::FTPLGR { lr } => FTPLGR::new(num_arms, *lr),
                Algorithms::ForcedExploration => ForcedExploration::new(num_arms),
                Algorithms::GIRO { num_pseudo_rewards } => GIRO::new(num_arms, *num_pseudo_rewards),
                Algorithms::Gradient => GradientBandit::new(num_arms, 0.1, false),
                Algorithms::GradientBaseline => GradientBandit::new(num_arms, 0.1, true),
                Algorithms::Greedy => Greedy::new(num_arms),
                Algorithms::HellingerUCB => HellingerUCB::new(num_arms),
                Algorithms::IRSFH { assumed_horizon } => IRSFH::new(num_arms, *assumed_horizon),
                Algorithms::KLMS => KLMS::new(num_arms),
                Algorithms::KLUCB => KLUCB::new(num_arms),
                Algorithms::LeastFailures => LeastFailures::new(num_arms),
                Algorithms::LilUCB { delta } => LilUCB::new(num_arms, *delta),
                Algorithms::MARS { delta } => MARS::new(num_arms, *delta),
                Algorithms::MBE => Mbe::new(num_arms),
                Algorithms::MOSSAnytime { alpha } => MOSSAnytime::new(num_arms, *alpha),
                Algorithms::NPTS => NPTS::new(num_arms),
                Algorithms::OptimisticTS => OptimisticTS::new(num_arms),
                Algorithms::POKER { assumed_horizon } => POKER::new(num_arms, *assumed_horizon),
                Algorithms::PHE { perturbation_scale } => PHE::new(num_arms, *perturbation_scale),
                Algorithms::Random => Random::new(num_arms),
                Algorithms::RS { aspiration } => RS::new(num_arms, *aspiration),
                Algorithms::SoftElim { w } => SoftElim::new(num_arms, *w),
                Algorithms::SoftSatisficing { aspiration } => SoftSatisficing::new(num_arms, *aspiration),
                Algorithms::RavenUCB { a0, b0, eps } => RavenUCB::new(num_arms, *a0, *b0, *eps),
                Algorithms::ReBoot { r } => ReBoot::new(num_arms, *r),
                Algorithms::ReUCB { a } => ReUCB::new(num_arms, *a),
                Algorithms::STS { epsilon } => STS::new(num_arms, *epsilon),
                Algorithms::TS => TS::new(num_arms),
                Algorithms::TSVHA => TSVHA::new(num_arms),
                Algorithms::TSUCB { samples } => TSUCB::new(num_arms, *samples),
                Algorithms::TsallisINF => TsallisINF::new(num_arms),
                Algorithms::UCB1 => UCB1::new(num_arms),
                Algorithms::UCB1Tuned => UCB1Tuned::new(num_arms),
                Algorithms::UCBDT { gamma } => UCBDT::new(num_arms, *gamma),
                Algorithms::UCBT => UCBT::new(num_arms),
                Algorithms::VarTS => VarTS::new(num_arms),
                Algorithms::VResBoot { init } => VResBoot::new(num_arms, *init),
                Algorithms::WB => WB::new(num_arms),
                Algorithms::WhittleApprox { beta } => WhittleApprox::new(num_arms, *beta),
                Algorithms::WRSDA { forced_exploration } => WRSDA::new(num_arms, *forced_exploration),
            })
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

    println!("{algorithm};{percent_optimal:0.2};{mean_regret:0.4};{mad:0.4};{elapsed:0.2}s");
}
