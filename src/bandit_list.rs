pub use crate::bandits::{
    bge::BGE, dirichlet_sampling::BDS, ebtci::EBTCI, eps_ts::EpsTS, giro::GIRO,
    gradient_bandit::GradientBandit, greedy::Greedy, kl_ucb::KLUCB, klms::KLMS, mbe::Mbe,
    npts::NPTS, phe::PHE, random::Random, ts::TS, ts_vha::TSVHA, tsucb::TSUCB, ucb1::UCB1,
    ucb1_tuned::UCB1Tuned, wr_sda::WRSDA, Algorithms,
};

pub const ALL_BANDITS: [Algorithms; 20] = [
    Algorithms::BDS,
    Algorithms::BGE,
    Algorithms::EBTCI,
    Algorithms::EpsTS,
    Algorithms::GIRO,
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
];
