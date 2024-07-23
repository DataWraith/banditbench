use rand::prelude::*;

pub mod baselines;
pub mod bootstrap;
pub mod dueling;
pub mod ts;
pub mod ucb;

pub mod bge;
pub mod code;
pub mod ebtci;
pub mod exp_ix;
pub mod forced_exploration;
pub mod gradient_bandit;
pub mod klms;
pub mod poker;
pub mod tsallis_inf;
pub mod tsucb;

pub use {
    baselines::eps_decreasing::EpsilonDecreasing, baselines::eps_greedy::EpsilonGreedy,
    baselines::etc::ETC, baselines::greedy::Greedy, baselines::random::Random, bge::BGE,
    bootstrap::bts::BTS, bootstrap::giro::GIRO, bootstrap::mbe::Mbe, bootstrap::phe::PHE,
    bootstrap::reboot::ReBoot, bootstrap::vresboot::VResBoot, code::CODE,
    dueling::dirichlet_sampling::BDS, dueling::wr_sda::WRSDA, ebtci::EBTCI, exp_ix::EXPIX,
    forced_exploration::ForcedExploration, gradient_bandit::GradientBandit, klms::KLMS,
    poker::POKER, ts::eps_ts::EpsTS, ts::npts::NPTS, ts::sts::STS, ts::ts::OptimisticTS,
    ts::ts::TS, ts::ts_vha::TSVHA, tsallis_inf::TsallisINF, tsucb::TSUCB,
    ucb::hellinger_ucb::HellingerUCB, ucb::kl_ucb::KLUCB, ucb::lilucb::LilUCB,
    ucb::moss_anytime::MOSSAnytime, ucb::ucb1::UCB1, ucb::ucb1_tuned::UCB1Tuned,
    ucb::ucb_dt::UCBDT, ucb::ucbt::UCBT,
};

pub enum Algorithms {
    BDS,
    BGE,
    BTS { replicates: usize },
    CODE { delta: f64 },
    EBTCI,
    EpsilonDecreasing { epsilon: f64 },
    EpsilonGreedy { epsilon: f64 },
    EpsTS,
    ETC { m: usize },
    EXPIX,
    ForcedExploration,
    GIRO { num_pseudo_rewards: f64 },
    Gradient,
    GradientBaseline,
    Greedy,
    HellingerUCB,
    KLMS,
    KLUCB,
    LilUCB { delta: f64 },
    MBE,
    MOSSAnytime { alpha: f64 },
    NPTS,
    OptimisticTS,
    PHE { perturbation_scale: f64 },
    POKER { assumed_horizon: usize },
    Random,
    ReBoot { r: f64 },
    STS { epsilon: f64 },
    TS,
    TSUCB { samples: usize },
    TSVHA,
    TsallisINF,
    UCB1,
    UCB1Tuned,
    UCBDT { gamma: f64 },
    UCBT,
    VResBoot { init: usize },
    WRSDA,
}

pub trait Bandit {
    fn pull(&mut self, rng: impl Rng) -> usize;
    fn update(&mut self, arm: usize, reward: bool, rng: impl Rng);
}

#[derive(Clone, Default)]
pub struct Arm {
    pub successes: usize,
    pub failures: usize,
}

impl Arm {
    pub fn mean(&self) -> f64 {
        (self.successes as f64) / (self.successes + self.failures).max(1) as f64
    }

    pub fn n(&self) -> usize {
        self.successes + self.failures
    }
}

impl std::fmt::Display for Algorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithms::BDS => write!(f, "Bounded Dirichlet Sampling"),
            Algorithms::BGE => write!(f, "Boltzmann-Gumbel Exploration"),
            Algorithms::BTS { replicates } => {
                write!(f, "Bootstrapped Thompson Sampling (J={})", replicates)
            }
            Algorithms::CODE { delta } => write!(f, "CODE (δ={:.3})", delta),
            Algorithms::EBTCI => write!(f, "EB-TCI"),
            Algorithms::EpsilonDecreasing { epsilon } => {
                write!(f, "ϵ-Decreasing (ϵ={:.3})", epsilon)
            }
            Algorithms::EpsilonGreedy { epsilon } => write!(f, "ϵ-Greedy (ϵ={:.3})", epsilon),
            Algorithms::EpsTS => write!(f, "ϵ-Exploring Thompson Sampling"),
            Algorithms::ETC { m } => write!(f, "ETC (m={})", m),
            Algorithms::EXPIX => write!(f, "EXP-IX"),
            Algorithms::ForcedExploration => write!(f, "Forced Exploration"),
            Algorithms::GIRO { num_pseudo_rewards } => {
                write!(f, "Garbage In, Reward Out (a={:.2})", num_pseudo_rewards)
            }
            Algorithms::Gradient => write!(f, "Gradient Bandit"),
            Algorithms::GradientBaseline => write!(f, "Gradient Bandit (with baseline)"),
            Algorithms::Greedy => write!(f, "Greedy"),
            Algorithms::HellingerUCB => write!(f, "Hellinger-UCB"),
            Algorithms::KLMS => write!(f, "Kullback-Leibler Maillard Sampling"),
            Algorithms::KLUCB => write!(f, "KL-UCB"),
            Algorithms::LilUCB { delta } => write!(f, "lil' UCB (δ={:.3})", delta),
            Algorithms::MBE => write!(f, "Multiplier Bootstrap-based Exploration"),
            Algorithms::MOSSAnytime { alpha } => write!(f, "MOSS-Anytime (α={:.2})", alpha),
            Algorithms::NPTS => write!(f, "Non-Parametric Thompson Sampling"),
            Algorithms::OptimisticTS => write!(f, "Optimistic Thompson Sampling"),
            Algorithms::PHE { perturbation_scale } => write!(
                f,
                "Perturbed-History Exploration (a={})",
                perturbation_scale
            ),
            Algorithms::POKER { assumed_horizon } => write!(f, "POKER (H={})", assumed_horizon),
            Algorithms::ReBoot { r } => write!(f, "ReBoot (r={:.2})", r),
            Algorithms::Random => write!(f, "Random"),
            Algorithms::STS { epsilon } => {
                write!(f, "Satisficing Thompson Sampling (ϵ={:.3})", epsilon)
            }
            Algorithms::TS => write!(f, "Thompson Sampling"),
            Algorithms::TSUCB { samples } => write!(f, "TS-UCB ({} samples)", samples),
            Algorithms::TSVHA => write!(
                f,
                "Thompson Sampling with Virtual Helping Agents (Combiner C3)"
            ),
            Algorithms::TsallisINF => write!(f, "Tsallis-INF"),
            Algorithms::UCB1 => write!(f, "UCB1"),
            Algorithms::UCB1Tuned => write!(f, "UCB1-Tuned"),
            Algorithms::UCBDT { gamma } => write!(f, "UCB-DT (γ={:.2})", gamma),
            Algorithms::UCBT => write!(f, "UCBT"),
            Algorithms::VResBoot { init } => {
                write!(f, "Vanilla Residual Bootstrap (init={})", init)
            }
            Algorithms::WRSDA => write!(f, "WR-SDA"),
        }
    }
}
