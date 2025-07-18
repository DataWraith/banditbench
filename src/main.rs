use clap::Parser;

use banditbench::{bandits::Algorithms, evaluate::evaluate_bandits};

#[derive(Parser)]
#[command(name = "Bandit Bench")]
#[command(version, about = None, long_about = None)]
struct Cli {
    /// Name of the experiment
    experiment: String,

    /// Algorithm to test
    algorithm: String,

    /// Horizon (number of rounds, T)
    #[arg(long, default_value_t = 500)]
    horizon: usize,

    /// Number of runs
    #[arg(short, long, default_value_t = 10001)]
    num_runs: usize,

    /// Starting seed value
    #[arg(short, long, default_value_t = 123_456)]
    seed: u64,
}

fn main() {
    let cli = Cli::parse();

    let algorithms = match cli.algorithm.to_ascii_lowercase().as_str() {
        "batch_ensemble" => vec![
            Algorithms::BatchEnsemble { multiplier: 0.0 },
            Algorithms::BatchEnsemble { multiplier: 1.0 },
            Algorithms::BatchEnsemble { multiplier: 2.0 },
            Algorithms::BatchEnsemble { multiplier: 4.0 },
            Algorithms::BatchEnsemble { multiplier: 8.0 },
        ],
        "bayes_ucb" => vec![
            Algorithms::BayesUCB { delta: 0.1 },
            Algorithms::BayesUCB { delta: 0.2 },
            Algorithms::BayesUCB { delta: 0.3 },
            Algorithms::BayesUCB { delta: 0.4 },
            Algorithms::BayesUCB { delta: 0.5 },
            Algorithms::BayesUCB { delta: 0.9 },
        ],
        "dirichlet_sampling" => vec![Algorithms::BDS],
        "bge" => vec![Algorithms::BGE],
        "brezzi_and_lai_approximation" => vec![
            Algorithms::BrezziLaiApprox { beta: 0.8 },
            Algorithms::BrezziLaiApprox { beta: 0.9 },
            Algorithms::BrezziLaiApprox { beta: 0.95 },
            Algorithms::BrezziLaiApprox { beta: 0.99 },
        ],
        "bts" => vec![
            Algorithms::BTS { replicates: 10 },
            Algorithms::BTS { replicates: 100 },
            Algorithms::BTS { replicates: 500 },
            Algorithms::BTS { replicates: 1000 },
        ],
        "code" => vec![
            Algorithms::CODE { delta: 0.05 },
            Algorithms::CODE { delta: 0.9 },
            Algorithms::CODE { delta: 0.99 },
        ],
        "ebtci" => vec![Algorithms::EBTCI],
        "eps_greedy" => vec![
            Algorithms::EpsilonGreedy { epsilon: 0.01 },
            Algorithms::EpsilonGreedy { epsilon: 0.02 },
            Algorithms::EpsilonGreedy { epsilon: 0.05 },
            Algorithms::EpsilonGreedy { epsilon: 0.1 },
        ],
        "eps_decreasing" => vec![
            Algorithms::EpsilonDecreasing { epsilon: 0.1 },
            Algorithms::EpsilonDecreasing { epsilon: 0.2 },
            Algorithms::EpsilonDecreasing { epsilon: 0.5 },
            Algorithms::EpsilonDecreasing { epsilon: 0.7 },
            Algorithms::EpsilonDecreasing { epsilon: 0.9 },
            Algorithms::EpsilonDecreasing { epsilon: 0.99 },
        ],
        "eps_ts" => vec![Algorithms::EpsTS],
        "eps_tsucb" => vec![
            Algorithms::EpsTSUCB { samples: 1 },
            Algorithms::EpsTSUCB { samples: 10 },
            Algorithms::EpsTSUCB { samples: 100 },
        ],
        "etc" => vec![
            Algorithms::ETC { m: 2 },
            Algorithms::ETC { m: 3 },
            Algorithms::ETC { m: 5 },
            Algorithms::ETC { m: 10 },
            Algorithms::ETC { m: 20 },
            Algorithms::ETC { m: 25 },
        ],
        "exp_ix" => vec![Algorithms::EXPIX],
        "forced_exploration" => vec![Algorithms::ForcedExploration],
        "ftpl_gr" => vec![
            Algorithms::FTPLGR { lr: 0.001 },
            Algorithms::FTPLGR { lr: 0.01 },
            Algorithms::FTPLGR { lr: 0.1 },
            Algorithms::FTPLGR { lr: 1.0 },
        ],
        "giro" => vec![
            Algorithms::GIRO {
                num_pseudo_rewards: 1.0,
            },
            Algorithms::GIRO {
                num_pseudo_rewards: 1.0 / 3.0,
            },
            Algorithms::GIRO {
                num_pseudo_rewards: 1.0 / 10.0,
            },
        ],
        "gradient_bandit" => vec![Algorithms::Gradient, Algorithms::GradientBaseline],
        "greedy" => vec![Algorithms::Greedy],
        "hellinger_ucb" => vec![Algorithms::HellingerUCB],
        "irs_fh" => vec![
            Algorithms::IRSFH { assumed_horizon: 1 },
            Algorithms::IRSFH { assumed_horizon: 2 },
            Algorithms::IRSFH { assumed_horizon: 3 },
            Algorithms::IRSFH { assumed_horizon: 4 },
            Algorithms::IRSFH { assumed_horizon: 5 },
            Algorithms::IRSFH {
                assumed_horizon: 10,
            },
            Algorithms::IRSFH {
                assumed_horizon: 25,
            },
        ],
        "klms" => vec![Algorithms::KLMS],
        "kl_ucb" => vec![Algorithms::KLUCB],
        "least_failures" => vec![Algorithms::LeastFailures],
        "lilucb" => vec![
            Algorithms::LilUCB { delta: 0.001 },
            Algorithms::LilUCB { delta: 0.01 },
            Algorithms::LilUCB { delta: 0.10 },
        ],
        "mars" => vec![
            Algorithms::MARS { delta: 1.0 / 1.0 },
            Algorithms::MARS { delta: 1.0 / 10.0 },
            Algorithms::MARS { delta: 1.0 / 100.0 },
            Algorithms::MARS { delta: 1.0 / 500.0 },
            Algorithms::MARS {
                delta: 1.0 / 1000.0,
            },
        ],
        "mbe" => vec![Algorithms::MBE],
        "moss_anytime" => vec![
            Algorithms::MOSSAnytime { alpha: -0.85 },
            Algorithms::MOSSAnytime { alpha: -0.5 },
            Algorithms::MOSSAnytime { alpha: -0.33 },
        ],
        "npts" => vec![Algorithms::NPTS],
        "ots" => vec![Algorithms::OptimisticTS],
        "phe" => vec![
            Algorithms::PHE {
                perturbation_scale: 1.1,
            },
            Algorithms::PHE {
                perturbation_scale: 2.1,
            },
            Algorithms::PHE {
                perturbation_scale: 5.1,
            },
        ],
        "poker" => vec![
            Algorithms::POKER { assumed_horizon: 1 },
            Algorithms::POKER { assumed_horizon: 5 },
            Algorithms::POKER {
                assumed_horizon: 10,
            },
            Algorithms::POKER {
                assumed_horizon: 25,
            },
            Algorithms::POKER {
                assumed_horizon: 50,
            },
            Algorithms::POKER {
                assumed_horizon: 100,
            },
            Algorithms::POKER {
                assumed_horizon: 250,
            },
        ],
        "raven_ucb" => vec![
            Algorithms::RavenUCB {
                a0: 0.5,
                b0: 0.5,
                eps: 1e-3,
            },
            Algorithms::RavenUCB {
                a0: 0.5,
                b0: 0.5,
                eps: 0.1,
            },
            Algorithms::RavenUCB {
                a0: 0.5,
                b0: 0.5,
                eps: 0.5,
            },
            Algorithms::RavenUCB {
                a0: 1.0,
                b0: 5.0,
                eps: 1e-3,
            },
            Algorithms::RavenUCB {
                a0: 1.0,
                b0: 5.0,
                eps: 0.1,
            },
            Algorithms::RavenUCB {
                a0: 1.0,
                b0: 5.0,
                eps: 0.5,
            },
            Algorithms::RavenUCB {
                a0: 0.5,
                b0: 10.0,
                eps: 1e-3,
            },
            Algorithms::RavenUCB {
                a0: 0.5,
                b0: 10.0,
                eps: 0.1,
            },
            Algorithms::RavenUCB {
                a0: 0.5,
                b0: 10.0,
                eps: 0.5,
            },
            Algorithms::RavenUCB {
                a0: 5.0,
                b0: 1.0,
                eps: 1e-3,
            },
            Algorithms::RavenUCB {
                a0: 5.0,
                b0: 1.0,
                eps: 0.1,
            },
            Algorithms::RavenUCB {
                a0: 5.0,
                b0: 1.0,
                eps: 0.5,
            },
        ],
        "reboot" => vec![
            Algorithms::ReBoot { r: 0.25 },
            Algorithms::ReBoot { r: 0.5 },
            Algorithms::ReBoot { r: 0.9 },
            Algorithms::ReBoot { r: 1.0 },
            Algorithms::ReBoot { r: 1.5 },
            Algorithms::ReBoot { r: 1.7 },
            Algorithms::ReBoot { r: 2.1 },
        ],
        "reucb" => vec![
            Algorithms::ReUCB { a: 1.0 },
            Algorithms::ReUCB { a: 1.5 },
            Algorithms::ReUCB { a: 2.0 },
        ],
        "random" => vec![Algorithms::Random],
        "rs" => vec![
            Algorithms::RS { aspiration: 0.0 },
            Algorithms::RS { aspiration: 0.1 },
            Algorithms::RS { aspiration: 0.25 },
            Algorithms::RS { aspiration: 0.5 },
            Algorithms::RS { aspiration: 0.65 },
            Algorithms::RS { aspiration: 0.75 },
            Algorithms::RS { aspiration: 0.9 },
            Algorithms::RS { aspiration: 0.99 },
        ],
        "soft_elim" => vec![
            Algorithms::SoftElim { theta: 0.01 },
            Algorithms::SoftElim { theta: 0.1 },
            Algorithms::SoftElim { theta: 0.25 },
            Algorithms::SoftElim { theta: 0.5 },
            Algorithms::SoftElim { theta: 1.0 },
            Algorithms::SoftElim { theta: 2.0 },
            Algorithms::SoftElim { theta: 5.0 },
        ],
        "softsatisficing" => vec![
            Algorithms::SoftSatisficing { aspiration: 0.1 },
            Algorithms::SoftSatisficing { aspiration: 0.25 },
            Algorithms::SoftSatisficing { aspiration: 0.5 },
            Algorithms::SoftSatisficing { aspiration: 0.65 },
            Algorithms::SoftSatisficing { aspiration: 0.75 },
            Algorithms::SoftSatisficing { aspiration: 0.9 },
            Algorithms::SoftSatisficing { aspiration: 0.99 },
        ],
        "sts" => vec![
            Algorithms::STS { epsilon: 0.005 },
            Algorithms::STS { epsilon: 0.010 },
            Algorithms::STS { epsilon: 0.050 },
            Algorithms::STS { epsilon: 0.100 },
        ],
        "ts" => vec![Algorithms::TS],
        "tsucb" => vec![
            Algorithms::TSUCB { samples: 1 },
            Algorithms::TSUCB { samples: 10 },
            Algorithms::TSUCB { samples: 100 },
        ],
        "ts_vha" => vec![Algorithms::TSVHA],
        "tsallis_inf" => vec![Algorithms::TsallisINF],
        "ucb1" => vec![Algorithms::UCB1],
        "ucb1_tuned" => vec![Algorithms::UCB1Tuned],
        "ucb_dt" => vec![
            Algorithms::UCBDT { gamma: 0.75 },
            Algorithms::UCBDT { gamma: 0.9 },
            Algorithms::UCBDT { gamma: 0.95 },
            Algorithms::UCBDT { gamma: 1.0 },
        ],
        "ucbt" => vec![Algorithms::UCBT],
        "vresboot" => vec![
            Algorithms::VResBoot { init: 0 },
            Algorithms::VResBoot { init: 1 },
            Algorithms::VResBoot { init: 5 },
        ],
        "whittle_approximation" => vec![
            Algorithms::WhittleApprox { beta: 0.5 },
            Algorithms::WhittleApprox { beta: 0.7 },
            Algorithms::WhittleApprox { beta: 0.9 },
            Algorithms::WhittleApprox { beta: 0.99 },
        ],
        "weighted_bootstrap" => vec![Algorithms::WB],
        "wr_sda" => vec![
            Algorithms::WRSDA {
                forced_exploration: false,
            },
            Algorithms::WRSDA {
                forced_exploration: true,
            },
        ],
        _ => panic!("No such algorithm: {}", cli.algorithm),
    };

    let arm_fn = match cli.experiment.to_ascii_lowercase().as_str() {
        "uniform" => experiments::uniform_arms,
        "half_range" => experiments::half_range,
        "hard" => experiments::hard,
        "beta" => experiments::beta_arms,
        "reverse_beta" => experiments::beta_reverse,
        _ => panic!("No experiment '{}' found", cli.experiment),
    };

    for algorithm in algorithms.iter() {
        evaluate_bandits(algorithm, cli.num_runs | 1, arm_fn, cli.seed, cli.horizon);
    }
}

mod experiments {
    use rand::prelude::*;
    use rand_distr::Beta;

    pub fn uniform_arms(seed: u64) -> Vec<f64> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        (0..10).map(|_| rng.gen_range(0.0..=1.0)).collect()
    }

    pub fn beta_arms(seed: u64) -> Vec<f64> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let dist = Beta::new(1.0, 8.0).unwrap();

        (0..10).map(|_| dist.sample(&mut rng)).collect()
    }

    pub fn beta_reverse(seed: u64) -> Vec<f64> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let dist = Beta::new(8.0, 1.0).unwrap();

        (0..10).map(|_| dist.sample(&mut rng)).collect()
    }

    pub fn half_range(seed: u64) -> Vec<f64> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        (0..10).map(|_| rng.gen_range(0.25..=0.75)).collect()
    }

    pub fn hard(seed: u64) -> Vec<f64> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let mut arms = vec![0.5; 10];
        arms[0] += 0.01;

        arms.shuffle(&mut rng);

        arms
    }
}
