use rand::prelude::*;

pub mod automata;
pub mod baselines;
pub mod bootstrap;
pub mod dueling;
pub mod gittins;
pub mod ts;
pub mod ucb;

pub mod bge;
pub mod code;
pub mod ebtci;
pub mod eps_tsucb;
pub mod exp_ix;
pub mod forced_exploration;
pub mod gradient_bandit;
pub mod klms;
pub mod poker;
pub mod rs;
pub mod soft_elim;
pub mod softsatisficing;
pub mod tsallis_inf;
pub mod tsucb;

pub use {
    automata::pfla::PFLA, baselines::eps_decreasing::EpsilonDecreasing,
    baselines::eps_greedy::EpsilonGreedy, baselines::etc::ETC, baselines::greedy::Greedy,
    baselines::least_failures::LeastFailures, baselines::random::Random, bge::BGE,
    bootstrap::bts::BTS, bootstrap::giro::GIRO, bootstrap::mars::MARS, bootstrap::mbe::Mbe,
    bootstrap::phe::PHE, bootstrap::reboot::ReBoot, bootstrap::vresboot::VResBoot,
    bootstrap::weighted_bootstrap::WB, code::CODE, dueling::dirichlet_sampling::BDS,
    dueling::wr_sda::WRSDA, ebtci::EBTCI, eps_tsucb::EpsTSUCB, exp_ix::EXPIX,
    forced_exploration::ForcedExploration, gittins::brezzi_and_lai_approximation::BrezziLaiApprox,
    gittins::whittle_approximation::WhittleApprox, gradient_bandit::GradientBandit, klms::KLMS,
    poker::POKER, rs::RS, soft_elim::SoftElim, softsatisficing::SoftSatisficing, ts::eps_ts::EpsTS,
    ts::irs_fh::IRSFH, ts::npts::NPTS, ts::sts::STS, ts::ts::OptimisticTS, ts::ts::TS,
    ts::ts_vha::TSVHA, tsallis_inf::TsallisINF, tsucb::TSUCB, ucb::bayes_ucb::BayesUCB,
    ucb::hellinger_ucb::HellingerUCB, ucb::kl_ucb::KLUCB, ucb::lilucb::LilUCB,
    ucb::moss_anytime::MOSSAnytime, ucb::reucb::ReUCB, ucb::ucb1::UCB1, ucb::ucb1_tuned::UCB1Tuned,
    ucb::ucb_dt::UCBDT, ucb::ucbt::UCBT,
};

pub enum Algorithms {
    BayesUCB { delta: f64 },
    BDS,
    BGE,
    BrezziLaiApprox { beta: f64 },
    BTS { replicates: usize },
    CODE { delta: f64 },
    EBTCI,
    EpsilonDecreasing { epsilon: f64 },
    EpsilonGreedy { epsilon: f64 },
    EpsTS,
    EpsTSUCB { samples: usize },
    ETC { m: usize },
    EXPIX,
    ForcedExploration,
    GIRO { num_pseudo_rewards: f64 },
    Gradient,
    GradientBaseline,
    Greedy,
    HellingerUCB,
    IRSFH { assumed_horizon: u64 },
    KLMS,
    KLUCB,
    LeastFailures,
    LilUCB { delta: f64 },
    MARS { delta: f64 },
    MBE,
    MOSSAnytime { alpha: f64 },
    NPTS,
    OptimisticTS,
    PFLA { n: usize },
    PHE { perturbation_scale: f64 },
    POKER { assumed_horizon: usize },
    Random,
    ReBoot { r: f64 },
    ReUCB { a: f64 },
    RS { aspiration: f64 },
    SoftElim { theta: f64 },
    SoftSatisficing { aspiration: f64 },
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
    WB,
    WhittleApprox { beta: f64 },
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
            Algorithms::BayesUCB { delta } => write!(f, "BayesUCB (δ={:.3})", delta),
            Algorithms::BDS => write!(f, "Bounded Dirichlet Sampling"),
            Algorithms::BGE => write!(f, "Boltzmann-Gumbel Exploration"),
            Algorithms::BrezziLaiApprox { beta } => write!(
                f,
                "Gittins Index -- Brezzi and Lai's Approximation (β={})",
                beta
            ),
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
            Algorithms::EpsTSUCB { samples } => {
                write!(f, "ϵ-Exploring TS-UCB ({} samples)", samples)
            }
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
            Algorithms::IRSFH { assumed_horizon } => write!(f, "IRS.FH (H={})", assumed_horizon),
            Algorithms::KLMS => write!(f, "Kullback-Leibler Maillard Sampling"),
            Algorithms::KLUCB => write!(f, "KL-UCB"),
            Algorithms::LeastFailures => write!(f, "Least Failures"),
            Algorithms::LilUCB { delta } => write!(f, "lil' UCB (δ={:.3})", delta),
            Algorithms::MARS { delta } => write!(f, "MARS (δ={:.3})", delta),
            Algorithms::MBE => write!(f, "Multiplier Bootstrap-based Exploration"),
            Algorithms::MOSSAnytime { alpha } => write!(f, "MOSS-Anytime (α={:.2})", alpha),
            Algorithms::NPTS => write!(f, "Non-Parametric Thompson Sampling"),
            Algorithms::OptimisticTS => write!(f, "Optimistic Thompson Sampling"),
            Algorithms::PFLA { n } => write!(f, "PFLA (n={})", n),
            Algorithms::PHE { perturbation_scale } => write!(
                f,
                "Perturbed-History Exploration (a={})",
                perturbation_scale
            ),
            Algorithms::POKER { assumed_horizon } => write!(f, "POKER (H={})", assumed_horizon),
            Algorithms::ReBoot { r } => write!(f, "ReBoot (r={:.2})", r),
            Algorithms::ReUCB { a } => write!(f, "ReUCB (a={:.2})", a),
            Algorithms::Random => write!(f, "Random"),
            Algorithms::RS { aspiration } => write!(f, "RS (a={:.2})", aspiration),
            Algorithms::SoftElim { theta } => write!(f, "SoftElim (θ={:.2})", theta),
            Algorithms::SoftSatisficing { aspiration } => {
                write!(f, "Softsatisficing (a={:.2})", aspiration)
            }
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
            Algorithms::WB => write!(f, "Weighted Bootstrap"),
            Algorithms::WhittleApprox { beta } => {
                write!(
                    f,
                    "Gittins Index -- Whittle's Approximation (β={:.2})",
                    beta
                )
            }
            Algorithms::WRSDA => write!(f, "WR-SDA"),
        }
    }
}
