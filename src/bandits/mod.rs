use rand::prelude::*;
use rand_distr::Beta;

pub mod baselines;
pub mod bootstrap;
pub mod dueling;
pub mod gittins;
pub mod ts;
pub mod ucb;

pub mod batch_ensemble;
pub mod bge;
pub mod code;
pub mod delightful_gradient_bandit;
pub mod ebtci;
pub mod eps_tsucb;
pub mod exp_ix;
pub mod forced_exploration;
pub mod ftpl_gr;
pub mod gradient_bandit;
pub mod klms;
pub mod poker;
pub mod rs;
pub mod soft_elim;
pub mod softsatisficing;
pub mod tsallis_inf;
pub mod tsucb;

pub use {
    baselines::eps_decreasing::EpsilonDecreasing, baselines::eps_greedy::EpsilonGreedy,
    baselines::etc::ETC, baselines::greedy::Greedy, baselines::least_failures::LeastFailures,
    baselines::random::Random, batch_ensemble::BatchEnsemble, bge::BGE, bootstrap::bts::BTS,
    bootstrap::giro::GIRO, bootstrap::mars::MARS, bootstrap::mbe::Mbe, bootstrap::phe::PHE,
    bootstrap::reboot::ReBoot, bootstrap::vresboot::VResBoot, bootstrap::weighted_bootstrap::WB,
    code::CODE, delightful_gradient_bandit::DelightfulGradientBandit,
    dueling::dirichlet_sampling::BDS, dueling::wr_sda::WRSDA, ebtci::EBTCI, eps_tsucb::EpsTSUCB,
    exp_ix::EXPIX, forced_exploration::ForcedExploration, ftpl_gr::FTPLGR,
    gittins::brezzi_and_lai_approximation::BrezziLaiApprox,
    gittins::whittle_approximation::WhittleApprox, gradient_bandit::GradientBandit, klms::KLMS,
    poker::POKER, rs::RS, soft_elim::SoftElim, softsatisficing::SoftSatisficing, ts::eps_ts::EpsTS,
    ts::irs_fh::IRSFH, ts::npts::NPTS, ts::ots::OptimisticTS, ts::sts::STS, ts::ts::TS,
    ts::ts_vha::TSVHA, ts::var_ts::VarTS, tsallis_inf::TsallisINF, tsucb::TSUCB,
    ucb::bayes_ucb::BayesUCB, ucb::hellinger_ucb::HellingerUCB, ucb::kl_ucb::KLUCB,
    ucb::lilucb::LilUCB, ucb::moss_anytime::MOSSAnytime, ucb::raven_ucb::RavenUCB,
    ucb::reucb::ReUCB, ucb::ucb1::UCB1, ucb::ucb1_tuned::UCB1Tuned, ucb::ucb_dt::UCBDT,
    ucb::ucbt::UCBT,
};

pub enum Algorithms {
    BatchEnsemble { multiplier: f64 },
    BayesUCB { delta: f64 },
    BDS,
    BGE,
    BrezziLaiApprox { beta: f64 },
    BTS { replicates: usize },
    CODE { delta: f64 },
    DelightfulGradient { lr: f64 },
    EBTCI,
    EpsilonDecreasing { epsilon: f64 },
    EpsilonGreedy { epsilon: f64 },
    EpsTS,
    EpsTSUCB { samples: usize },
    ETC { m: usize },
    EXPIX,
    FTPLGR { lr: f64 },
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
    PHE { perturbation_scale: f64 },
    POKER { assumed_horizon: usize },
    Random,
    RavenUCB { a0: f64, b0: f64, eps: f64 },
    ReBoot { r: f64 },
    ReUCB { a: f64 },
    RS { aspiration: f64 },
    SoftElim { w: f64 },
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
    VarTS,
    VResBoot { init: usize },
    WB,
    WhittleApprox { beta: f64 },
    WRSDA { forced_exploration: bool },
}

pub trait Bandit: std::fmt::Display {
    fn pull(&mut self, rng: &mut impl Rng) -> usize;
    fn update(&mut self, arm: usize, reward: bool, rng: &mut impl Rng);
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

    pub fn update(&mut self, reward: bool) {
        if reward {
            self.successes += 1;
        } else {
            self.failures += 1;
        }
    }

    pub fn beta(&self) -> Beta<f64> {
        Beta::new(self.successes as f64 + 1.0, self.failures as f64 + 1.0).unwrap()
    }
}
