use rand::prelude::*;

pub mod bge;
pub mod dirichlet_sampling;
pub mod ebtci;
pub mod eps_ts;
pub mod forced_exploration;
pub mod giro;
pub mod gradient_bandit;
pub mod greedy;
pub mod kl_ucb;
pub mod klms;
pub mod mbe;
pub mod npts;
pub mod phe;
pub mod random;
pub mod reboot;
pub mod sts;
pub mod ts;
pub mod ts_vha;
pub mod tsallis_inf;
pub mod tsucb;
pub mod ucb1;
pub mod ucb1_tuned;
pub mod vresboot;
pub mod wr_sda;

pub enum Algorithms {
    BDS,
    BGE,
    EBTCI,
    EpsTS,
    ForcedExploration,
    GIRO { num_pseudo_rewards: f64 },
    Gradient,
    GradientBaseline,
    Greedy,
    KLMS,
    KLUCB,
    MBE,
    NPTS,
    PHE { perturbation_scale: f64 },
    Random,
    ReBoot { r: f64 },
    STS { epsilon: f64 },
    TS,
    TSUCB { samples: usize },
    TSVHA,
    TsallisINF,
    UCB1,
    UCB1Tuned,
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
            Algorithms::EBTCI => write!(f, "EB-TCI"),
            Algorithms::EpsTS => write!(f, "ϵ-Exploring Thompson Sampling"),
            Algorithms::ForcedExploration => write!(f, "Forced Exploration"),
            Algorithms::GIRO { num_pseudo_rewards } => {
                write!(f, "Garbage In, Reward Out (a={:.2})", num_pseudo_rewards)
            }
            Algorithms::Gradient => write!(f, "Gradient Bandit"),
            Algorithms::GradientBaseline => write!(f, "Gradient Bandit (with baseline)"),
            Algorithms::Greedy => write!(f, "Greedy"),
            Algorithms::KLMS => write!(f, "Kullback-Leibler Maillard Sampling"),
            Algorithms::KLUCB => write!(f, "KL-UCB"),
            Algorithms::MBE => write!(f, "Multiplier Bootstrap-based Exploration"),
            Algorithms::NPTS => write!(f, "Non-Parametric Thompson Sampling"),
            Algorithms::PHE { perturbation_scale } => write!(
                f,
                "Perturbed-History Exploration (a={})",
                perturbation_scale
            ),
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
            Algorithms::VResBoot { init } => {
                write!(f, "Vanilla Residual Bootstrap (init={})", init)
            }
            Algorithms::WRSDA => write!(f, "WR-SDA"),
        }
    }
}
