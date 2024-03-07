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
        "bds" => vec![Algorithms::BDS],
        "bge" => vec![Algorithms::BGE],
        "ebtci" => vec![Algorithms::EBTCI],
        "epsts" => vec![Algorithms::EpsTS],
        "fe" => vec![Algorithms::ForcedExploration],
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
        "gradient" => vec![Algorithms::Gradient, Algorithms::GradientBaseline],
        "greedy" => vec![Algorithms::Greedy],
        "klms" => vec![Algorithms::KLMS],
        "klucb" => vec![Algorithms::KLUCB],
        "mbe" => vec![Algorithms::MBE],
        "npts" => vec![Algorithms::NPTS],
        "ots" => vec![Algorithms::OptimisticTS],
        "phe" => vec![
            Algorithms::PHE {
                perturbation_scale: 1.1,
            },
            Algorithms::PHE {
                perturbation_scale: 2.1,
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
        "random" => vec![Algorithms::Random],
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
        "tsvha" => vec![Algorithms::TSVHA],
        "tsallisinf" => vec![Algorithms::TsallisINF],
        "ucb1" => vec![Algorithms::UCB1],
        "ucbdt" => vec![
            Algorithms::UCBDT { gamma: 0.75 },
            Algorithms::UCBDT { gamma: 0.9 },
            Algorithms::UCBDT { gamma: 0.95 },
            Algorithms::UCBDT { gamma: 1.0 },
        ],
        "ucb1tuned" => vec![Algorithms::UCB1Tuned],
        "vresboot" => vec![
            Algorithms::VResBoot { init: 0 },
            Algorithms::VResBoot { init: 1 },
            Algorithms::VResBoot { init: 5 },
        ],
        "wrsda" => vec![Algorithms::WRSDA],
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
        evaluate_bandits(algorithm, cli.num_runs | 1, arm_fn, cli.horizon);
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
