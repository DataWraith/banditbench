pub use crate::bandits::{
    bge::BGE, dirichlet_sampling::BDS, ebtci::EBTCI, eps_ts::EpsTS,
    forced_exploration::ForcedExploration, giro::GIRO, gradient_bandit::GradientBandit,
    greedy::Greedy, kl_ucb::KLUCB, klms::KLMS, mbe::Mbe, npts::NPTS, phe::PHE, random::Random,
    reboot::ReBoot, reboot::ReBootSlow, sts::STS, ts::TS, ts_vha::TSVHA, tsucb::TSUCB, ucb1::UCB1,
    ucb1_tuned::UCB1Tuned, wr_sda::WRSDA, Algorithms,
};

pub const ALL_BANDITS: [Algorithms; 32] = [
    Algorithms::BDS,
    Algorithms::BGE,
    Algorithms::EBTCI,
    Algorithms::EpsTS,
    Algorithms::ForcedExploration,
    Algorithms::GIRO {
        num_pseudo_rewards: 1.0,
    },
    Algorithms::GIRO {
        num_pseudo_rewards: 1.0 / 3.0,
    },
    Algorithms::GIRO {
        num_pseudo_rewards: 1.0 / 10.0,
    },
    Algorithms::Gradient,
    Algorithms::GradientBaseline,
    Algorithms::Greedy,
    Algorithms::KLMS,
    Algorithms::KLUCB,
    Algorithms::MBE,
    Algorithms::NPTS,
    Algorithms::PHE {
        perturbation_scale: 1.1,
    },
    Algorithms::PHE {
        perturbation_scale: 2.1,
    },
    Algorithms::OptimisticReBoot,
    Algorithms::ReBoot,
    Algorithms::ReBootSlow,
    Algorithms::Random,
    Algorithms::STS { epsilon: 0.005 },
    Algorithms::STS { epsilon: 0.01 },
    Algorithms::STS { epsilon: 0.05 },
    Algorithms::TS,
    Algorithms::TSUCB { samples: 1 },
    Algorithms::TSUCB { samples: 10 },
    Algorithms::TSUCB { samples: 100 },
    Algorithms::TSVHA,
    Algorithms::UCB1,
    Algorithms::UCB1Tuned,
    Algorithms::WRSDA,
];
