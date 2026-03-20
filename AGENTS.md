# Bandit Bench

**Project Type:** Rust

**Description:** A benchmarking suite for Bernoulli Multi-Armed Bandit algorithms. Tests 55+ algorithms on short-horizon problems (500 pulls) with binary rewards.

## Architecture

- **Core Trait:** `Bandit` (in `src/bandits/mod.rs`) defines the interface: `pull()` selects an arm, `update()` learns from rewards
- **Algorithm Config:** `Algorithms` enum holds algorithm variants with parameters; `main.rs` maps CLI args to algorithm configs
- **Evaluation:** `evaluate_bandit()` in `lib.rs` runs single trials; `evaluate_bandits()` in `evaluate.rs` handles parallel batch execution with progress bars
- **Parallelism:** Uses `rayon` for parallel simulation runs across multiple seeds

## Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | CLI entrypoint (clap), algorithm selection mapping |
| `src/lib.rs` | Core `Bandit` trait, single-trial evaluation logic |
| `src/evaluate.rs` | Parallel batch evaluation with progress bars |
| `src/bandits/mod.rs` | `Algorithms` enum, `Bandit` trait, `Arm` helper struct |
| `src/bandits/` | Algorithm implementations (~43 files) |
| `experiments/` | Experiment orchestration scripts |

## Algorithm Categories

- `baselines/` - Greedy, ε-Greedy, Random, etc.
- `bootstrap/` - Bootstrapped Thompson Sampling variants
- `ts/` - Thompson Sampling family
- `ucb/` - Upper Confidence Bound variants
- `dueling/`, `gittins/` - Specialized approaches
- Root `bandits/` - Other algorithms (gradient, elimination, etc.)

## Dependencies

- `rand` / `rand_distr` - Random number generation, Beta distribution
- `statrs` - Statistical distributions
- `rayon` - Data parallelism
- `indicatif` - Progress bars
- `clap` - CLI parsing
- `ordered-float` - Float ordering for comparisons
