# Bandit Bench

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

This project is a small, unscientific benchmark of algorithms for the Bernoulli
Multi-Armed Bandit. It benchmarks my specific use-case of short-horizon problems
(500 arm pulls) with Bernoulli rewards (i.e., either there is a reward or there
is not, with a given probability).

Algorithms are only included in the benchmark if

- They are easy to implement
- They do not depend on the time horizon explicitly
- They do not need (much) parameter tuning

## Algorithms

### Baselines

- Random Baseline (chooses arms randomly)
- Greedy Baseline (chooses the arm with the maximum average reward)
- ϵ-Greedy
- ϵ-Decreasing
- Explore Then Commit
- Least Failures (then most successes, then random)

### Bootstrap-based

- [Bootstrapped Thompson Sampling](https://arxiv.org/abs/1410.4009)
- [Garbage In, Reward Out](http://proceedings.mlr.press/v97/kveton19a/kveton19a.pdf) (PDF)
- [MARS](https://proceedings.neurips.cc/paper_files/paper/2023/hash/b84adff45775e92a45f0cd87c37f5ce9-Abstract-Conference.html)
- [Multiplier Bootstrap-based Exploration](https://arxiv.org/abs/2302.01543)
- [Perturbed-History Exploration](https://arxiv.org/abs/1902.10089)
- [ReBoot](https://arxiv.org/abs/2002.08436)
- Vanilla Residual Bootstrap
- [WB](https://arxiv.org/abs/1805.09793)

### Dueling-based

- [Bounded Dirichlet Sampling](https://arxiv.org/abs/2111.09724)
- [WR-SDA](https://arxiv.org/abs/2010.14323)

### Approximations of the Gittins Index

- Whittle's approximation (works best for large simulation horizon)
- Brezzi and Lai's approximation

### Thompson Sampling-based

- [ϵ-Exploring Thompson Sampling](https://proceedings.mlr.press/v202/jin23b/jin23b.pdf) (PDF)
- [Information Relaxation Sampling (Finite Horizon)](https://arxiv.org/abs/1902.04251) (simplified version that uses an assumed, fixed horizon)
- [Non-Parametric Thompson Sampling](https://proceedings.mlr.press/v117/riou20a.html)
- Optimistic Thompson Sampling
- [Satisficing Thompson Sampling](https://arxiv.org/abs/1704.09028)
- [Thompson Sampling with Virtual Helping Agents (C3)](https://arxiv.org/abs/2209.08197)
- Thompson Sampling
- [VarTS](https://arxiv.org/abs/2303.09033)

### Upper Confidence Bound-based

- [BayesUCB](https://arxiv.org/abs/2306.09136)
- [Hellinger-UCB](https://arxiv.org/abs/2404.10207)
- [KL-UCB](https://arxiv.org/abs/1102.2490)
- [lil' UCB](https://arxiv.org/abs/1312.7308)
- [MOSS-anytime](http://proceedings.mlr.press/v48/degenne16.html)
- [RAVEN-UCB](https://arxiv.org/abs/2506.02933)
- [ReUCB](https://arxiv.org/abs/2106.12200)
- [UCB1](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [UCB1-Tuned](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [UCB-DT](https://arxiv.org/abs/2110.02690)
- [UCBT](https://arxiv.org/abs/2102.05263)

### Other

- [Batch Ensemble for MAB](https://arxiv.org/abs/2409.08570)
- [Boltzmann-Gumbel Exploration](https://arxiv.org/abs/1705.10257)
- [CODE](https://arxiv.org/abs/2310.14751)
- [EB-TCI](https://arxiv.org/abs/2206.05979)
- [EXP-IX](https://arxiv.org/abs/1506.03271)
- [Forced Exploration](https://arxiv.org/abs/2312.07285)
- [Gradient Bandit](https://arxiv.org/abs/2402.17235)
- [Kullback-Leibler Maillard Sampling](https://arxiv.org/abs/2304.14989)
- [POKER](https://link.springer.com/chapter/10.1007/11564096_42) (with fixed Horizon)
- [Risk-sensitive Satisficing (RS)](https://doi.org/10.1016/j.biosystems.2019.02.009)
- [SoftElim](https://arxiv.org/abs/2002.06772)
- [Softsatisficing](https://doi.org/10.1016/j.biosystems.2022.104633)
- [Tsallis-INF](https://arxiv.org/abs/1807.07623)
- [TS-UCB](https://arxiv.org/abs/2006.06372)

## Results

The following table shows the average rank and runtime of each algorithm when
considering the five experiments further down in this file.

<!-- `> cat experiments/aggregated_ranks.md` -->

<!-- BEGIN mdsh -->

| Algorithm                                                   | Average Rank | Average Time (seconds) |
| ----------------------------------------------------------- | ------------ | ---------------------- |
| Batch Ensemble for MAB (m=0)                                | 15.6         | 0                      |
| IRS.FH (H=2)                                                | 16.8         | 1                      |
| IRS.FH (H=3)                                                | 19.2         | 1                      |
| UCB-DT (γ=0.90)                                             | 19.2         | 3                      |
| UCB-DT (γ=0.95)                                             | 19.8         | 3                      |
| IRS.FH (H=1)                                                | 20.6         | 1                      |
| UCB-DT (γ=0.75)                                             | 21.8         | 3                      |
| TS-UCB (100 samples)                                        | 24.4         | 68                     |
| UCB-DT (γ=1.00)                                             | 24.6         | 3                      |
| ϵ-Exploring TS-UCB (1 samples)                              | 26.6         | 0                      |
| Batch Ensemble for MAB (m=1)                                | 27.6         | 0                      |
| ϵ-Exploring TS-UCB (10 samples)                             | 27.8         | 1                      |
| IRS.FH (H=4)                                                | 28.0         | 1                      |
| ϵ-Exploring TS-UCB (100 samples)                            | 28.0         | 8                      |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.99)    | 28.2         | 0                      |
| UCB-DT (γ=0.50)                                             | 28.8         | 3                      |
| Gittins Index -- Whittle's Approximation (β=0.99)           | 30.6         | 0                      |
| SoftElim (w=0.10)                                           | 30.6         | 0                      |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.1)                         | 31.4         | 0                      |
| MOSS-Anytime (α=-0.85)                                      | 31.6         | 0                      |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.5)                         | 31.8         | 0                      |
| TS-UCB (10 samples)                                         | 33.2         | 7                      |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.001)                       | 33.4         | 0                      |
| IRS.FH (H=5)                                                | 36.6         | 1                      |
| Gittins Index -- Whittle's Approximation (β=0.90)           | 36.8         | 0                      |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.5)                          | 37.4         | 0                      |
| BayesUCB (δ=0.300)                                          | 38.4         | 0                      |
| ϵ-Decreasing (ϵ=0.990)                                      | 38.8         | 0                      |
| POKER (H=5)                                                 | 39.0         | 0                      |
| POKER (H=10)                                                | 39.2         | 0                      |
| ReUCB (a=2.00)                                              | 39.8         | 1                      |
| POKER (H=1)                                                 | 40.2         | 0                      |
| Greedy                                                      | 40.2         | 0                      |
| BayesUCB (δ=0.400)                                          | 40.4         | 0                      |
| ReUCB (a=1.50)                                              | 40.4         | 1                      |
| BayesUCB (δ=0.200)                                          | 40.6         | 0                      |
| ϵ-Decreasing (ϵ=0.900)                                      | 40.6         | 0                      |
| ReUCB (a=1.00)                                              | 41.2         | 1                      |
| ϵ-Decreasing (ϵ=0.700)                                      | 42.0         | 0                      |
| CODE (δ=0.990)                                              | 42.4         | 0                      |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.1)                          | 43.2         | 0                      |
| MOSS-Anytime (α=-0.50)                                      | 44.0         | 0                      |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) | 44.4         | 14                     |
| ϵ-Greedy (ϵ=0.010)                                          | 45.0         | 0                      |
| ϵ-Greedy (ϵ=0.020)                                          | 45.4         | 0                      |
| UCB-DT (γ=0.25)                                             | 45.6         | 3                      |
| Gittins Index -- Whittle's Approximation (β=0.70)           | 48.0         | 0                      |
| BayesUCB (δ=0.500)                                          | 48.4         | 0                      |
| Gittins Index -- Whittle's Approximation (β=0.50)           | 49.2         | 0                      |
| TS-UCB (1 samples)                                          | 49.8         | 1                      |
| ϵ-Greedy (ϵ=0.050)                                          | 49.8         | 0                      |
| POKER (H=25)                                                | 50.4         | 0                      |
| ϵ-Decreasing (ϵ=0.500)                                      | 53.0         | 0                      |
| MOSS-Anytime (α=-0.33)                                      | 53.2         | 0                      |
| IRS.FH (H=10)                                               | 53.4         | 1                      |
| ϵ-Exploring Thompson Sampling                               | 53.4         | 0                      |
| POKER (H=50)                                                | 54.8         | 0                      |
| POKER (H=100)                                               | 56.6         | 0                      |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.001)                        | 57.4         | 0                      |
| RAVEN-UCB (a0=1, b0=5, eps=0.5)                             | 60.0         | 0                      |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.9)     | 60.0         | 0                      |
| RAVEN-UCB (a0=1, b0=5, eps=0.1)                             | 62.2         | 0                      |
| ϵ-Greedy (ϵ=0.100)                                          | 62.2         | 0                      |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.8)     | 62.8         | 0                      |
| WR-SDA (forced_exploration=true)                            | 63.0         | 2                      |
| UCBT                                                        | 65.4         | 0                      |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.95)    | 65.6         | 0                      |
| BayesUCB (δ=0.100)                                          | 66.6         | 0                      |
| IRS.FH (H=25)                                               | 67.0         | 2                      |
| Forced Exploration                                          | 68.8         | 0                      |
| BayesUCB (δ=0.900)                                          | 69.0         | 0                      |
| RAVEN-UCB (a0=1, b0=5, eps=0.001)                           | 69.4         | 0                      |
| WR-SDA (forced_exploration=false)                           | 69.8         | 1                      |
| SoftElim (w=0.50)                                           | 72.2         | 0                      |
| ReBoot (r=0.25)                                             | 72.6         | 0                      |
| MARS (δ=0.100)                                              | 73.6         | 0                      |
| POKER (H=250)                                               | 75.2         | 0                      |
| Batch Ensemble for MAB (m=2)                                | 76.6         | 0                      |
| VarTS                                                       | 79.0         | 0                      |
| Thompson Sampling                                           | 79.4         | 1                      |
| Weighted Bootstrap                                          | 80.2         | 3                      |
| Satisficing Thompson Sampling (ϵ=0.005)                     | 80.8         | 1                      |
| ReBoot (r=0.50)                                             | 81.6         | 0                      |
| Satisficing Thompson Sampling (ϵ=0.010)                     | 82.2         | 1                      |
| Bootstrapped Thompson Sampling (J=10)                       | 82.4         | 0                      |
| KL-UCB                                                      | 84.0         | 8                      |
| Hellinger-UCB                                               | 84.0         | 2                      |
| Vanilla Residual Bootstrap (init=0)                         | 84.8         | 0                      |
| Bootstrapped Thompson Sampling (J=500)                      | 85.2         | 4                      |
| Bootstrapped Thompson Sampling (J=1000)                     | 86.4         | 9                      |
| Bootstrapped Thompson Sampling (J=100)                      | 86.6         | 1                      |
| Non-Parametric Thompson Sampling                            | 87.4         | 4                      |
| Multiplier Bootstrap-based Exploration                      | 87.8         | 6                      |
| CODE (δ=0.900)                                              | 91.0         | 0                      |
| UCB1-Tuned                                                  | 91.4         | 0                      |
| Garbage In, Reward Out (a=0.10)                             | 92.2         | 1                      |
| Bounded Dirichlet Sampling                                  | 95.6         | 2                      |
| Satisficing Thompson Sampling (ϵ=0.050)                     | 95.8         | 1                      |
| UCB-DT (γ=0.10)                                             | 96.4         | 3                      |
| Vanilla Residual Bootstrap (init=1)                         | 96.6         | 0                      |
| MARS (δ=0.010)                                              | 97.6         | 1                      |
| ϵ-Decreasing (ϵ=0.200)                                      | 98.2         | 0                      |
| Kullback-Leibler Maillard Sampling                          | 98.6         | 1                      |
| Perturbed-History Exploration (a=1.1)                       | 101.8        | 1                      |
| RS (a=0.50)                                                 | 102.4        | 0                      |
| RS (a=0.65)                                                 | 102.8        | 0                      |
| EB-TCI                                                      | 103.2        | 0                      |
| Softsatisficing (a=0.65)                                    | 107.2        | 0                      |
| Batch Ensemble for MAB (m=4)                                | 108.0        | 0                      |
| Tsallis-INF                                                 | 109.8        | 1                      |
| Batch Ensemble for MAB (m=8)                                | 110.6        | 0                      |
| ReBoot (r=0.90)                                             | 111.0        | 0                      |
| MARS (δ=0.001)                                              | 111.4        | 11                     |
| MARS (δ=0.002)                                              | 111.6        | 6                      |
| Garbage In, Reward Out (a=0.33)                             | 112.2        | 1                      |
| RS (a=0.90)                                                 | 113.2        | 0                      |
| MARS (δ=1.000)                                              | 113.4        | 0                      |
| Satisficing Thompson Sampling (ϵ=0.100)                     | 114.2        | 1                      |
| lil' UCB (δ=0.100)                                          | 114.4        | 0                      |
| ETC (m=10)                                                  | 114.6        | 0                      |
| RS (a=0.75)                                                 | 115.6        | 0                      |
| FTPL-GR (lr=1.000)                                          | 115.8        | 4                      |
| Vanilla Residual Bootstrap (init=5)                         | 117.4        | 0                      |
| ReBoot (r=1.00)                                             | 117.8        | 0                      |
| SoftElim (w=1.00)                                           | 119.2        | 0                      |
| Perturbed-History Exploration (a=2.1)                       | 121.6        | 1                      |
| Softsatisficing (a=0.75)                                    | 122.2        | 0                      |
| Softsatisficing (a=0.90)                                    | 124.2        | 0                      |
| ETC (m=20)                                                  | 125.6        | 0                      |
| FTPL-GR (lr=0.100)                                          | 125.8        | 4                      |
| ϵ-Decreasing (ϵ=0.100)                                      | 125.8        | 0                      |
| lil' UCB (δ=0.010)                                          | 128.6        | 0                      |
| Gradient Bandit                                             | 128.8        | 0                      |
| ETC (m=5)                                                   | 129.0        | 0                      |
| Garbage In, Reward Out (a=1.00)                             | 129.6        | 1                      |
| Gradient Bandit (with baseline)                             | 130.2        | 0                      |
| Boltzmann-Gumbel Exploration                                | 130.4        | 0                      |
| Softsatisficing (a=0.50)                                    | 130.8        | 0                      |
| ETC (m=25)                                                  | 131.8        | 0                      |
| ReBoot (r=1.50)                                             | 132.0        | 0                      |
| RS (a=0.99)                                                 | 133.4        | 0                      |
| RS (a=0.25)                                                 | 133.4        | 0                      |
| lil' UCB (δ=0.001)                                          | 134.6        | 0                      |
| ReBoot (r=1.70)                                             | 135.4        | 0                      |
| Softsatisficing (a=0.25)                                    | 136.2        | 0                      |
| Softsatisficing (a=0.99)                                    | 136.4        | 0                      |
| RS (a=0.10)                                                 | 136.4        | 0                      |
| Least Failures                                              | 137.0        | 0                      |
| Perturbed-History Exploration (a=5.1)                       | 138.2        | 1                      |
| UCB1                                                        | 142.0        | 0                      |
| ReBoot (r=2.10)                                             | 142.0        | 0                      |
| EXP-IX                                                      | 142.8        | 0                      |
| ETC (m=3)                                                   | 144.8        | 0                      |
| Softsatisficing (a=0.10)                                    | 145.6        | 0                      |
| RAVEN-UCB (a0=5, b0=1, eps=0.5)                             | 146.6        | 0                      |
| RAVEN-UCB (a0=5, b0=1, eps=0.1)                             | 147.2        | 0                      |
| RAVEN-UCB (a0=5, b0=1, eps=0.001)                           | 148.6        | 0                      |
| SoftElim (w=2.00)                                           | 148.8        | 0                      |
| ETC (m=2)                                                   | 150.0        | 0                      |
| UCB-DT (γ=0.02)                                             | 152.8        | 2                      |
| RS (a=0.00)                                                 | 152.8        | 0                      |
| SoftElim (w=2.83)                                           | 156.2        | 0                      |
| FTPL-GR (lr=0.010)                                          | 156.8        | 4                      |
| FTPL-GR (lr=0.001)                                          | 163.6        | 4                      |
| CODE (δ=0.050)                                              | 164.8        | 0                      |
| Random                                                      | 164.8        | 0                      |

<!-- END mdsh -->

## Data

### Uniform

This experiment uses 10 arms, with the means sampled uniformly from the interval
[0, 1]. This is a relatively easy instance, because there is likely to be a
single best arm that is easy to find. This is reflected in the %-Optimal column,
where the best algorithms reach over 2/3 pull rate of the optimal arm.

<details>
<summary>Results</summary>

<!-- `> cat experiments/uniform.md` -->

<!-- BEGIN mdsh -->

| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| Gittins Index -- Whittle's Approximation (β=0.99)           |     65.59 |       16.1524 |                             6.1313 | 0.21s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.99)    |     68.73 |       16.2172 |                             3.6835 | 0.42s  |
| BayesUCB (δ=0.300)                                          |     68.64 |       16.4619 |                             5.4296 | 0.20s  |
| Batch Ensemble for MAB (m=0)                                |     71.10 |       16.5066 |                             2.6067 | 0.05s  |
| Batch Ensemble for MAB (m=1)                                |     70.97 |       16.7903 |                             2.9368 | 0.08s  |
| TS-UCB (100 samples)                                        |     71.96 |       16.8902 |                             3.3855 | 64.82s |
| IRS.FH (H=2)                                                |     71.14 |       16.9375 |                             2.7279 | 1.35s  |
| IRS.FH (H=3)                                                |     72.29 |       16.9395 |                             3.0432 | 1.15s  |
| BayesUCB (δ=0.400)                                          |     64.26 |       17.1313 |                             6.9164 | 0.19s  |
| TS-UCB (10 samples)                                         |     72.56 |       17.2502 |                             3.7256 | 7.66s  |
| Gittins Index -- Whittle's Approximation (β=0.90)           |     61.89 |       17.4329 |                             6.6927 | 0.22s  |
| BayesUCB (δ=0.200)                                          |     71.21 |       17.5846 |                             4.0029 | 0.20s  |
| IRS.FH (H=4)                                                |     72.57 |       17.6097 |                             3.5134 | 1.27s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     69.94 |       17.8335 |                             2.8808 | 6.15s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     70.16 |       17.9184 |                             2.9572 | 0.74s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     70.36 |       18.0420 |                             3.0933 | 0.14s  |
| IRS.FH (H=1)                                                |     68.89 |       18.0936 |                             2.5509 | 1.28s  |
| UCB-DT (γ=1.00)                                             |     69.93 |       18.1466 |                             2.5287 | 2.63s  |
| UCB-DT (γ=0.95)                                             |     72.44 |       18.1946 |                             2.4725 | 2.69s  |
| UCB-DT (γ=0.75)                                             |     72.50 |       18.1962 |                             2.5172 | 2.76s  |
| UCB-DT (γ=0.90)                                             |     72.42 |       18.2016 |                             2.4807 | 2.65s  |
| IRS.FH (H=5)                                                |     72.59 |       18.3624 |                             4.0320 | 1.36s  |
| UCB-DT (γ=0.50)                                             |     72.85 |       18.3997 |                             2.5814 | 2.65s  |
| MOSS-Anytime (α=-0.85)                                      |     69.71 |       18.8113 |                             2.5659 | 0.18s  |
| SoftElim (w=0.10)                                           |     68.33 |       18.8211 |                             2.5745 | 0.36s  |
| CODE (δ=0.990)                                              |     68.91 |       18.9329 |                             2.9569 | 0.16s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.5)                         |     70.40 |       19.0812 |                             2.7529 | 0.27s  |
| POKER (H=10)                                                |     65.48 |       19.3526 |                             3.2000 | 0.30s  |
| POKER (H=5)                                                 |     66.34 |       19.3558 |                             2.7035 | 0.32s  |
| ReUCB (a=2.00)                                              |     68.28 |       19.4080 |                             2.6887 | 0.94s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.1)                         |     70.19 |       19.4646 |                             2.7603 | 0.27s  |
| TS-UCB (1 samples)                                          |     71.83 |       19.5545 |                             5.3564 | 0.78s  |
| ReUCB (a=1.50)                                              |     67.94 |       19.5633 |                             2.7023 | 0.95s  |
| POKER (H=1)                                                 |     66.55 |       19.5748 |                             2.5553 | 0.29s  |
| ReUCB (a=1.00)                                              |     67.60 |       19.7102 |                             2.7106 | 0.99s  |
| Greedy                                                      |     66.26 |       19.7129 |                             2.5470 | 0.06s  |
| BayesUCB (δ=0.500)                                          |     58.32 |       20.1518 |                             9.3444 | 0.19s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.001)                       |     69.57 |       20.5446 |                             2.8610 | 0.22s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     66.35 |       20.7765 |                             2.7735 | 0.14s  |
| BayesUCB (δ=0.100)                                          |     68.95 |       20.9900 |                             3.7246 | 0.16s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     63.16 |       21.1041 |                             6.1932 | 24.02s |
| ϵ-Greedy (ϵ=0.010)                                          |     66.18 |       21.1769 |                             2.8588 | 0.08s  |
| IRS.FH (H=10)                                               |     71.83 |       21.2519 |                             5.4688 | 1.45s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     66.48 |       21.2824 |                             2.8492 | 0.14s  |
| POKER (H=25)                                                |     61.38 |       21.3386 |                             6.6186 | 0.30s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.5)                          |     68.76 |       22.2566 |                             2.9656 | 0.23s  |
| MOSS-Anytime (α=-0.50)                                      |     70.74 |       22.4582 |                             2.7088 | 0.21s  |
| ϵ-Greedy (ϵ=0.020)                                          |     65.99 |       22.7752 |                             3.1672 | 0.11s  |
| RS (a=0.75)                                                 |     57.47 |       23.4688 |                            11.0929 | 0.17s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.1)                          |     68.14 |       23.4949 |                             3.2027 | 0.23s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     66.55 |       23.6847 |                             3.3687 | 0.13s  |
| WR-SDA (forced_exploration=false)                           |     66.87 |       23.8280 |                             5.0922 | 1.12s  |
| WR-SDA (forced_exploration=true)                            |     69.42 |       24.2844 |                             5.2129 | 1.27s  |
| IRS.FH (H=25)                                               |     70.09 |       24.4452 |                             6.0015 | 1.60s  |
| MOSS-Anytime (α=-0.33)                                      |     69.75 |       24.4536 |                             2.6909 | 0.21s  |
| Gittins Index -- Whittle's Approximation (β=0.70)           |     51.65 |       24.9930 |                            11.3705 | 0.22s  |
| UCB-DT (γ=0.25)                                             |     71.98 |       25.1053 |                             2.6535 | 2.68s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.5)                             |     65.05 |       25.4596 |                             3.6551 | 0.24s  |
| POKER (H=50)                                                |     56.21 |       25.6788 |                             9.6913 | 0.31s  |
| SoftElim (w=0.50)                                           |     64.43 |       26.0903 |                             4.2092 | 0.40s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.1)                             |     64.72 |       26.1332 |                             3.7397 | 0.24s  |
| ϵ-Greedy (ϵ=0.050)                                          |     65.45 |       27.3929 |                             4.0210 | 0.11s  |
| ϵ-Exploring Thompson Sampling                               |     62.82 |       27.9018 |                             9.2377 | 0.15s  |
| UCBT                                                        |     65.40 |       28.7984 |                             4.0759 | 0.07s  |
| Thompson Sampling                                           |     66.16 |       28.8956 |                             7.1444 | 0.60s  |
| Weighted Bootstrap                                          |     66.08 |       28.9034 |                             7.1124 | 3.05s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     65.94 |       29.0318 |                             7.1008 | 0.94s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     65.61 |       29.3229 |                             7.0179 | 0.83s  |
| KL-UCB                                                      |     66.78 |       29.6304 |                             7.3837 | 7.45s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.001)                        |     65.19 |       29.6926 |                             4.6441 | 0.23s  |
| ReBoot (r=0.25)                                             |     61.18 |       30.3599 |                             5.2731 | 0.25s  |
| Softsatisficing (a=0.75)                                    |     49.75 |       30.3853 |                            17.8608 | 0.11s  |
| Gittins Index -- Whittle's Approximation (β=0.50)           |     46.19 |       30.4565 |                            14.2565 | 0.18s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.001)                           |     62.48 |       30.6351 |                             4.5496 | 0.28s  |
| CODE (δ=0.900)                                              |     54.94 |       30.6423 |                             6.5536 | 0.17s  |
| Batch Ensemble for MAB (m=2)                                |     62.49 |       30.7481 |                             7.8970 | 0.14s  |
| POKER (H=100)                                               |     51.57 |       30.8991 |                            12.6895 | 0.31s  |
| Hellinger-UCB                                               |     63.89 |       31.0005 |                             7.0702 | 2.31s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     65.55 |       31.3306 |                             4.6232 | 0.13s  |
| UCB1-Tuned                                                  |     62.03 |       31.6747 |                             3.6906 | 0.15s  |
| Vanilla Residual Bootstrap (init=0)                         |     59.99 |       33.1442 |                             5.4073 | 0.23s  |
| Non-Parametric Thompson Sampling                            |     63.70 |       33.7962 |                             7.1820 | 3.80s  |
| RS (a=0.65)                                                 |     44.01 |       33.8259 |                            18.1643 | 0.18s  |
| ReBoot (r=0.50)                                             |     58.58 |       34.0829 |                             5.9224 | 0.30s  |
| Bounded Dirichlet Sampling                                  |     63.86 |       34.1647 |                             7.1345 | 1.84s  |
| MARS (δ=0.100)                                              |     64.47 |       34.5294 |                             4.8042 | 0.32s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     57.19 |       35.0506 |                             6.7983 | 0.85s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.95)    |     42.16 |       35.0542 |                            18.6679 | 0.40s  |
| ϵ-Greedy (ϵ=0.100)                                          |     63.98 |       35.8380 |                             5.3322 | 0.11s  |
| Multiplier Bootstrap-based Exploration                      |     60.70 |       36.1612 |                             4.2418 | 6.28s  |
| Kullback-Leibler Maillard Sampling                          |     59.67 |       37.5162 |                             8.3979 | 0.47s  |
| Perturbed-History Exploration (a=1.1)                       |     56.96 |       37.8929 |                             5.6711 | 0.70s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.9)     |     40.26 |       38.2054 |                            20.8866 | 0.40s  |
| POKER (H=250)                                               |     46.27 |       38.6838 |                            15.5508 | 0.30s  |
| Garbage In, Reward Out (a=0.10)                             |     57.65 |       38.7302 |                             5.2772 | 0.79s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.8)     |     39.50 |       39.7859 |                            22.1131 | 0.36s  |
| Softsatisficing (a=0.65)                                    |     40.21 |       39.8959 |                            24.8725 | 0.09s  |
| BayesUCB (δ=0.900)                                          |     39.28 |       40.0985 |                            22.3598 | 0.19s  |
| FTPL-GR (lr=1.000)                                          |     58.80 |       40.3823 |                            10.4546 | 4.78s  |
| Vanilla Residual Bootstrap (init=1)                         |     59.43 |       40.6304 |                             4.7837 | 0.28s  |
| Bootstrapped Thompson Sampling (J=500)                      |     40.59 |       41.9370 |                            21.7066 | 4.45s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     40.88 |       41.9668 |                            21.1936 | 8.36s  |
| Bootstrapped Thompson Sampling (J=100)                      |     40.77 |       42.3584 |                            21.7453 | 1.13s  |
| Bootstrapped Thompson Sampling (J=10)                       |     39.55 |       42.8224 |                            21.8677 | 0.37s  |
| VarTS                                                       |     64.30 |       42.9062 |                             5.1435 | 0.43s  |
| RS (a=0.90)                                                 |     61.99 |       43.3480 |                            18.0732 | 0.17s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     44.13 |       44.2992 |                            10.4673 | 0.95s  |
| lil' UCB (δ=0.100)                                          |     52.19 |       44.8365 |                             5.5606 | 0.29s  |
| Tsallis-INF                                                 |     54.25 |       46.4787 |                             5.9697 | 1.05s  |
| Softsatisficing (a=0.90)                                    |     56.49 |       46.6220 |                            27.5781 | 0.24s  |
| Forced Exploration                                          |     62.89 |       46.6666 |                             6.2607 | 0.08s  |
| ReBoot (r=0.90)                                             |     52.24 |       47.2795 |                             6.7367 | 0.28s  |
| UCB-DT (γ=0.10)                                             |     62.72 |       47.7019 |                             5.0109 | 3.01s  |
| Garbage In, Reward Out (a=0.33)                             |     51.74 |       49.2706 |                             5.5459 | 1.05s  |
| Vanilla Residual Bootstrap (init=5)                         |     55.69 |       50.7442 |                             6.1208 | 0.27s  |
| SoftElim (w=1.00)                                           |     47.68 |       51.6367 |                             6.0049 | 0.39s  |
| ReBoot (r=1.00)                                             |     49.90 |       51.8800 |                             6.7533 | 0.25s  |
| MARS (δ=0.010)                                              |     54.83 |       53.5390 |                             6.1628 | 1.38s  |
| Batch Ensemble for MAB (m=4)                                |     48.14 |       53.9552 |                            22.0397 | 0.20s  |
| EB-TCI                                                      |     42.82 |       55.0174 |                            15.7714 | 0.29s  |
| MARS (δ=0.001)                                              |     50.10 |       55.4818 |                            10.4818 | 12.12s |
| RS (a=0.50)                                                 |     32.37 |       55.6085 |                            36.1138 | 0.18s  |
| Perturbed-History Exploration (a=2.1)                       |     47.44 |       56.5448 |                             6.0521 | 0.92s  |
| ETC (m=10)                                                  |     47.32 |       56.6956 |                            11.0554 | 0.15s  |
| FTPL-GR (lr=0.100)                                          |     50.69 |       57.1522 |                             7.0720 | 4.05s  |
| MARS (δ=0.002)                                              |     50.18 |       59.5941 |                             8.8231 | 5.49s  |
| lil' UCB (δ=0.010)                                          |     44.08 |       62.1486 |                             6.5312 | 0.31s  |
| Softsatisficing (a=0.50)                                    |     28.97 |       63.5094 |                            43.9773 | 0.08s  |
| MARS (δ=1.000)                                              |     37.18 |       65.5059 |                            21.5650 | 0.09s  |
| Garbage In, Reward Out (a=1.00)                             |     43.03 |       66.4802 |                             6.9482 | 1.26s  |
| Boltzmann-Gumbel Exploration                                |     43.87 |       68.9250 |                             6.5817 | 0.32s  |
| Gradient Bandit                                             |     46.48 |       69.6675 |                             9.5534 | 0.37s  |
| Gradient Bandit (with baseline)                             |     48.72 |       70.6839 |                             6.1066 | 0.46s  |
| ReBoot (r=1.50)                                             |     40.44 |       72.1794 |                             8.1305 | 0.27s  |
| lil' UCB (δ=0.001)                                          |     39.18 |       73.8291 |                             8.0325 | 0.28s  |
| ETC (m=5)                                                   |     27.93 |       78.7963 |                            24.1796 | 0.16s  |
| ReBoot (r=1.70)                                             |     37.41 |       79.4522 |                             8.9230 | 0.28s  |
| Batch Ensemble for MAB (m=8)                                |     34.92 |       80.6517 |                            46.0772 | 0.27s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     50.82 |       81.7548 |                            11.1762 | 0.11s  |
| Perturbed-History Exploration (a=5.1)                       |     36.06 |       83.3539 |                             9.5119 | 1.07s  |
| RS (a=0.99)                                                 |     42.93 |       84.4523 |                            29.9659 | 0.18s  |
| ETC (m=20)                                                  |     49.52 |       85.1694 |                            11.9964 | 0.15s  |
| UCB1                                                        |     34.52 |       86.8474 |                            10.2054 | 0.15s  |
| Least Failures                                              |     40.55 |       88.7625 |                            28.1293 | 0.07s  |
| Softsatisficing (a=0.99)                                    |     39.79 |       89.3024 |                            27.8456 | 0.40s  |
| ReBoot (r=2.10)                                             |     32.31 |       92.8131 |                            10.7156 | 0.34s  |
| EXP-IX                                                      |     31.87 |       95.7830 |                            13.0250 | 0.50s  |
| UCB-DT (γ=0.02)                                             |     34.36 |       96.2940 |                            10.6775 | 2.33s  |
| SoftElim (w=2.00)                                           |     28.77 |       96.6009 |                            12.3509 | 0.40s  |
| ETC (m=3)                                                   |     22.30 |       98.5252 |                            27.0722 | 0.15s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.5)                             |     28.73 |      104.0006 |                            13.6507 | 0.26s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.1)                             |     28.72 |      104.0303 |                            13.6481 | 0.26s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.001)                           |     28.63 |      104.3277 |                            13.7403 | 0.26s  |
| RS (a=0.25)                                                 |     19.47 |      104.5549 |                            74.4142 | 0.19s  |
| ETC (m=25)                                                  |     41.95 |      105.2629 |                            14.8396 | 0.15s  |
| ETC (m=2)                                                   |     20.21 |      110.5641 |                            26.8868 | 0.11s  |
| Softsatisficing (a=0.25)                                    |     18.13 |      114.1714 |                            81.1272 | 0.06s  |
| SoftElim (w=2.83)                                           |     21.40 |      124.1101 |                            16.8242 | 0.41s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     35.59 |      127.2145 |                            17.7947 | 0.05s  |
| RS (a=0.10)                                                 |     16.10 |      132.1474 |                            91.9926 | 0.17s  |
| Softsatisficing (a=0.10)                                    |     15.69 |      135.0154 |                            93.5393 | 0.05s  |
| FTPL-GR (lr=0.010)                                          |     20.49 |      142.8645 |                            20.4026 | 4.16s  |
| RS (a=0.00)                                                 |     15.09 |      144.1846 |                            97.5086 | 0.13s  |
| CODE (δ=0.050)                                              |     10.94 |      187.9726 |                            24.8420 | 0.16s  |
| FTPL-GR (lr=0.001)                                          |     10.91 |      196.2329 |                            29.3352 | 4.10s  |
| Random                                                      |     10.01 |      204.0160 |                            30.3495 | 0.01s  |

<!-- END mdsh -->

</details>

### Half-Range

This experiment uses 10 arms, with the means sampled uniformly from the interval
[0.25, 0.75]. This is a harder instance, because the arms are closer together
and thus harder to distinguish.

This experiment was taken from the GIRO paper.

<details>
<summary>Results</summary>

<!-- `> cat experiments/half_range.md` -->

<!-- BEGIN mdsh -->

| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| Batch Ensemble for MAB (m=0)                                |     44.01 |       24.3162 |                             7.5674 | 0.05s  |
| UCB-DT (γ=0.50)                                             |     44.22 |       24.6659 |                             6.6225 | 2.73s  |
| IRS.FH (H=2)                                                |     43.85 |       24.7422 |                             7.8222 | 1.24s  |
| IRS.FH (H=3)                                                |     45.28 |       25.0047 |                             6.6326 | 1.31s  |
| Gittins Index -- Whittle's Approximation (β=0.99)           |     44.09 |       25.0082 |                             7.8533 | 0.22s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.9)     |     40.05 |       25.2717 |                            12.2698 | 0.40s  |
| UCB-DT (γ=0.90)                                             |     43.02 |       25.6120 |                             7.2004 | 2.71s  |
| UCB-DT (γ=0.95)                                             |     43.00 |       25.6319 |                             7.1816 | 2.74s  |
| UCB-DT (γ=0.75)                                             |     43.05 |       25.6700 |                             7.2075 | 2.74s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.99)    |     41.68 |       25.7326 |                             9.2085 | 0.43s  |
| Gittins Index -- Whittle's Approximation (β=0.90)           |     41.53 |       25.7390 |                             9.3584 | 0.22s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.95)    |     40.51 |       25.7582 |                            11.6365 | 0.40s  |
| Gittins Index -- Whittle's Approximation (β=0.50)           |     40.01 |       25.7986 |                            11.1271 | 0.19s  |
| BayesUCB (δ=0.900)                                          |     39.16 |       25.8336 |                            13.1050 | 0.19s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.8)     |     38.83 |       26.1183 |                            12.9800 | 0.36s  |
| BayesUCB (δ=0.500)                                          |     42.67 |       26.1689 |                             8.4137 | 0.19s  |
| Gittins Index -- Whittle's Approximation (β=0.70)           |     40.02 |       26.4219 |                            10.1272 | 0.21s  |
| BayesUCB (δ=0.400)                                          |     43.04 |       26.4233 |                             8.1311 | 0.19s  |
| BayesUCB (δ=0.300)                                          |     44.61 |       26.4883 |                             7.0859 | 0.19s  |
| IRS.FH (H=4)                                                |     44.59 |       26.4971 |                             6.7171 | 1.28s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.001)                       |     42.52 |       26.5203 |                             7.2100 | 0.20s  |
| Batch Ensemble for MAB (m=1)                                |     43.55 |       26.6555 |                             8.8838 | 0.09s  |
| UCB-DT (γ=0.25)                                             |     44.49 |       26.7171 |                             5.7916 | 2.72s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     44.11 |       26.7250 |                             8.7506 | 10.59s |
| IRS.FH (H=1)                                                |     39.73 |       26.9134 |                             9.8115 | 1.13s  |
| TS-UCB (100 samples)                                        |     45.02 |       26.9156 |                             6.2201 | 67.71s |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.1)                         |     41.15 |       27.0983 |                             8.1856 | 0.26s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.5)                          |     42.23 |       27.1170 |                             6.8959 | 0.24s  |
| RS (a=0.65)                                                 |     44.47 |       27.1789 |                            13.8551 | 0.18s  |
| BayesUCB (δ=0.200)                                          |     45.32 |       27.2126 |                             6.1882 | 0.20s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     42.08 |       27.2128 |                             8.1379 | 0.15s  |
| MOSS-Anytime (α=-0.85)                                      |     40.04 |       27.3181 |                             8.7262 | 0.18s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.5)                         |     40.90 |       27.4143 |                             8.4479 | 0.28s  |
| MOSS-Anytime (α=-0.50)                                      |     44.05 |       27.4891 |                             5.4358 | 0.22s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     41.55 |       27.5450 |                             8.1473 | 0.74s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     41.08 |       27.6722 |                             8.2739 | 6.21s  |
| CODE (δ=0.990)                                              |     39.41 |       27.7728 |                            10.1499 | 0.17s  |
| TS-UCB (10 samples)                                         |     44.54 |       27.9224 |                             5.9276 | 7.17s  |
| SoftElim (w=0.10)                                           |     38.57 |       27.9881 |                             9.7007 | 0.36s  |
| UCB-DT (γ=1.00)                                             |     38.52 |       28.0522 |                             9.8213 | 2.69s  |
| IRS.FH (H=5)                                                |     43.60 |       28.0814 |                             6.8634 | 1.31s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     38.24 |       28.1487 |                             9.6145 | 0.14s  |
| Greedy                                                      |     37.83 |       28.2076 |                             9.9996 | 0.08s  |
| Softsatisficing (a=0.65)                                    |     41.45 |       28.2436 |                            15.4217 | 0.18s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     38.32 |       28.3069 |                             9.4761 | 0.14s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     39.35 |       28.3077 |                             8.7988 | 0.13s  |
| POKER (H=1)                                                 |     37.76 |       28.3667 |                            10.1082 | 0.29s  |
| POKER (H=5)                                                 |     37.76 |       28.3800 |                            10.0953 | 0.37s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.1)                          |     41.48 |       28.3881 |                             6.9847 | 0.24s  |
| POKER (H=10)                                                |     37.74 |       28.4050 |                            10.0473 | 0.34s  |
| ϵ-Greedy (ϵ=0.010)                                          |     38.03 |       28.4793 |                             9.7905 | 0.08s  |
| ReUCB (a=2.00)                                              |     38.15 |       28.5063 |                             9.6122 | 0.95s  |
| ReUCB (a=1.50)                                              |     38.07 |       28.5077 |                             9.7464 | 0.94s  |
| ReUCB (a=1.00)                                              |     37.96 |       28.5231 |                             9.8749 | 0.92s  |
| ϵ-Greedy (ϵ=0.020)                                          |     38.36 |       28.6900 |                             9.4808 | 0.11s  |
| POKER (H=25)                                                |     37.49 |       28.8412 |                             9.4550 | 0.32s  |
| ϵ-Greedy (ϵ=0.050)                                          |     39.46 |       29.3486 |                             8.7084 | 0.11s  |
| Bootstrapped Thompson Sampling (J=10)                       |     38.57 |       29.4073 |                            13.9756 | 0.36s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     40.91 |       29.4333 |                             7.5048 | 0.13s  |
| MOSS-Anytime (α=-0.33)                                      |     42.29 |       29.8866 |                             5.9957 | 0.23s  |
| POKER (H=100)                                               |     38.92 |       29.9131 |                             6.5647 | 0.34s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.5)                             |     41.51 |       30.0406 |                             6.8188 | 0.25s  |
| BayesUCB (δ=0.100)                                          |     42.74 |       30.3237 |                             6.0061 | 0.16s  |
| POKER (H=50)                                                |     36.98 |       30.4262 |                             8.3416 | 0.33s  |
| RS (a=0.50)                                                 |     34.65 |       30.4958 |                            17.8406 | 0.18s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.1)                             |     41.13 |       30.6400 |                             6.5227 | 0.25s  |
| ϵ-Exploring Thompson Sampling                               |     40.14 |       30.7659 |                             8.9988 | 0.16s  |
| Bootstrapped Thompson Sampling (J=500)                      |     38.36 |       30.8943 |                            13.6813 | 4.30s  |
| Bootstrapped Thompson Sampling (J=100)                      |     38.23 |       30.9704 |                            13.6387 | 1.14s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     37.93 |       31.2238 |                            13.7505 | 8.38s  |
| SoftElim (w=0.50)                                           |     41.35 |       31.4556 |                             6.6870 | 0.40s  |
| ϵ-Greedy (ϵ=0.100)                                          |     40.16 |       31.5381 |                             7.6639 | 0.11s  |
| TS-UCB (1 samples)                                          |     41.21 |       31.8313 |                             6.2230 | 0.79s  |
| IRS.FH (H=10)                                               |     41.13 |       32.0134 |                             6.6906 | 1.51s  |
| UCBT                                                        |     41.92 |       32.0754 |                             5.3843 | 0.08s  |
| MARS (δ=0.100)                                              |     40.72 |       32.4299 |                             6.8526 | 0.33s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.001)                        |     39.29 |       32.7851 |                             6.8408 | 0.29s  |
| Forced Exploration                                          |     41.72 |       33.1699 |                             5.7046 | 0.09s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.001)                           |     39.57 |       33.2162 |                             6.5485 | 0.28s  |
| Softsatisficing (a=0.50)                                    |     31.09 |       33.7174 |                            20.6161 | 0.09s  |
| WR-SDA (forced_exploration=true)                            |     40.23 |       33.8608 |                             6.5333 | 1.88s  |
| POKER (H=250)                                               |     37.22 |       33.9079 |                             8.0820 | 0.34s  |
| WR-SDA (forced_exploration=false)                           |     37.74 |       34.3702 |                             7.8470 | 1.73s  |
| VarTS                                                       |     40.86 |       34.4084 |                             5.8058 | 0.46s  |
| IRS.FH (H=25)                                               |     38.87 |       35.4924 |                             6.7519 | 1.86s  |
| CODE (δ=0.900)                                              |     35.87 |       35.7202 |                            11.4984 | 0.18s  |
| UCB1-Tuned                                                  |     38.36 |       36.0304 |                             5.8517 | 0.16s  |
| ReBoot (r=0.25)                                             |     35.81 |       36.8892 |                             8.1828 | 0.19s  |
| Vanilla Residual Bootstrap (init=0)                         |     35.10 |       38.0391 |                             7.9288 | 0.17s  |
| UCB-DT (γ=0.10)                                             |     38.19 |       38.4314 |                             6.2251 | 2.67s  |
| Multiplier Bootstrap-based Exploration                      |     36.05 |       38.7066 |                             7.0003 | 5.91s  |
| ReBoot (r=0.50)                                             |     34.21 |       39.5480 |                             8.2009 | 0.22s  |
| ETC (m=10)                                                  |     33.45 |       40.0881 |                            11.7950 | 0.15s  |
| Hellinger-UCB                                               |     36.12 |       40.4295 |                             6.1041 | 2.40s  |
| Weighted Bootstrap                                          |     35.00 |       40.5410 |                             7.4857 | 3.01s  |
| Thompson Sampling                                           |     35.01 |       40.5420 |                             7.5125 | 0.60s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     34.96 |       40.5786 |                             7.5540 | 0.80s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     34.87 |       40.6461 |                             7.5447 | 0.81s  |
| Batch Ensemble for MAB (m=2)                                |     35.57 |       40.7384 |                             9.1522 | 0.17s  |
| Garbage In, Reward Out (a=0.10)                             |     33.73 |       42.0945 |                             7.6013 | 0.97s  |
| Perturbed-History Exploration (a=1.1)                       |     33.49 |       42.3004 |                             7.7267 | 0.81s  |
| KL-UCB                                                      |     34.54 |       42.7149 |                             6.2245 | 7.90s  |
| EB-TCI                                                      |     30.56 |       42.8317 |                             9.3319 | 0.36s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     32.52 |       43.1108 |                             8.0902 | 0.92s  |
| Non-Parametric Thompson Sampling                            |     33.09 |       43.6865 |                             7.5605 | 3.75s  |
| Vanilla Residual Bootstrap (init=1)                         |     32.88 |       43.7710 |                             7.4509 | 0.21s  |
| MARS (δ=0.010)                                              |     33.86 |       44.2200 |                             6.6462 | 1.37s  |
| Bounded Dirichlet Sampling                                  |     32.79 |       44.7466 |                             7.9659 | 2.21s  |
| Tsallis-INF                                                 |     32.35 |       45.6862 |                             8.4068 | 1.08s  |
| lil' UCB (δ=0.100)                                          |     31.70 |       46.4287 |                             6.7023 | 0.28s  |
| MARS (δ=1.000)                                              |     27.30 |       46.6224 |                            23.1260 | 0.09s  |
| Kullback-Leibler Maillard Sampling                          |     29.69 |       47.8324 |                             8.4744 | 0.61s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     27.45 |       48.1450 |                            10.2207 | 0.98s  |
| Garbage In, Reward Out (a=0.33)                             |     30.11 |       48.1458 |                             8.0648 | 1.17s  |
| MARS (δ=0.001)                                              |     30.62 |       48.3961 |                             7.5977 | 11.13s |
| ReBoot (r=0.90)                                             |     29.34 |       48.4181 |                             8.4845 | 0.23s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     33.79 |       49.1413 |                             7.5396 | 0.12s  |
| MARS (δ=0.002)                                              |     30.50 |       49.4331 |                             7.3296 | 5.89s  |
| SoftElim (w=1.00)                                           |     28.58 |       49.5066 |                             8.4513 | 0.41s  |
| ETC (m=5)                                                   |     21.32 |       50.0278 |                            17.6885 | 0.15s  |
| ReBoot (r=1.00)                                             |     27.89 |       50.9352 |                             8.6898 | 0.23s  |
| ETC (m=20)                                                  |     31.24 |       51.1732 |                             8.6350 | 0.16s  |
| Perturbed-History Exploration (a=2.1)                       |     27.91 |       52.2188 |                             8.4423 | 0.99s  |
| Softsatisficing (a=0.75)                                    |     30.32 |       52.6410 |                            16.5585 | 0.36s  |
| FTPL-GR (lr=0.100)                                          |     28.27 |       52.7166 |                            10.1011 | 4.12s  |
| Vanilla Residual Bootstrap (init=5)                         |     28.26 |       53.2834 |                             8.4062 | 0.23s  |
| RS (a=0.75)                                                 |     29.84 |       54.6692 |                            13.4306 | 0.19s  |
| Gradient Bandit                                             |     29.48 |       54.9276 |                             8.8193 | 0.40s  |
| Gradient Bandit (with baseline)                             |     29.07 |       55.8207 |                             8.2645 | 0.46s  |
| Batch Ensemble for MAB (m=4)                                |     26.48 |       55.8781 |                            10.9358 | 0.27s  |
| ETC (m=25)                                                  |     32.18 |       56.3820 |                             8.2546 | 0.15s  |
| FTPL-GR (lr=1.000)                                          |     26.06 |       56.8336 |                            12.0625 | 4.16s  |
| lil' UCB (δ=0.010)                                          |     25.83 |       56.9410 |                             8.2814 | 0.31s  |
| Garbage In, Reward Out (a=1.00)                             |     25.12 |       57.7304 |                             9.1152 | 1.15s  |
| Boltzmann-Gumbel Exploration                                |     25.61 |       58.0539 |                             8.8928 | 0.33s  |
| ReBoot (r=1.50)                                             |     22.85 |       61.0890 |                             9.6647 | 0.22s  |
| lil' UCB (δ=0.001)                                          |     22.85 |       62.7995 |                             9.1698 | 0.31s  |
| ReBoot (r=1.70)                                             |     21.38 |       64.4112 |                            10.0761 | 0.23s  |
| Perturbed-History Exploration (a=5.1)                       |     21.44 |       65.8492 |                            10.0502 | 1.02s  |
| UCB1                                                        |     20.42 |       68.0927 |                            10.1489 | 0.16s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     24.60 |       68.8686 |                             9.8576 | 0.07s  |
| ReBoot (r=2.10)                                             |     19.16 |       69.7726 |                            10.8419 | 0.22s  |
| ETC (m=3)                                                   |     15.41 |       69.9994 |                            18.3348 | 0.14s  |
| EXP-IX                                                      |     19.28 |       71.2582 |                            11.2795 | 0.44s  |
| RS (a=0.25)                                                 |     15.97 |       71.7134 |                            48.5357 | 0.18s  |
| SoftElim (w=2.00)                                           |     17.72 |       72.9077 |                            11.2645 | 0.40s  |
| Batch Ensemble for MAB (m=8)                                |     17.28 |       74.4122 |                            19.8696 | 0.33s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.5)                             |     17.71 |       74.8132 |                            11.3884 | 0.27s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.1)                             |     17.70 |       74.8207 |                            11.3883 | 0.28s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.001)                           |     17.67 |       74.8915 |                            11.3848 | 0.27s  |
| Softsatisficing (a=0.90)                                    |     17.73 |       76.2770 |                            12.8748 | 0.42s  |
| RS (a=0.90)                                                 |     17.16 |       77.8704 |                            12.5705 | 0.19s  |
| Softsatisficing (a=0.25)                                    |     14.53 |       78.0274 |                            53.4058 | 0.07s  |
| UCB-DT (γ=0.02)                                             |     17.08 |       78.8644 |                            11.9782 | 2.38s  |
| ETC (m=2)                                                   |     15.27 |       80.4676 |                            18.0151 | 0.11s  |
| Softsatisficing (a=0.99)                                    |     15.71 |       81.5688 |                            13.1278 | 0.43s  |
| RS (a=0.99)                                                 |     15.39 |       82.4437 |                            12.9414 | 0.19s  |
| Least Failures                                              |     15.39 |       82.4443 |                            12.9451 | 0.09s  |
| SoftElim (w=2.83)                                           |     14.47 |       82.7775 |                            12.5511 | 0.40s  |
| RS (a=0.10)                                                 |     13.03 |       84.6250 |                            56.6531 | 0.16s  |
| FTPL-GR (lr=0.010)                                          |     14.69 |       85.0457 |                            12.8256 | 4.23s  |
| RS (a=0.00)                                                 |     12.85 |       85.6737 |                            57.3841 | 0.13s  |
| Softsatisficing (a=0.10)                                    |     12.81 |       86.5606 |                            58.7008 | 0.04s  |
| FTPL-GR (lr=0.001)                                          |     10.46 |      100.0294 |                            14.8858 | 4.12s  |
| Random                                                      |     10.01 |      102.0080 |                            15.1748 | 0.01s  |
| CODE (δ=0.050)                                              |     10.00 |      102.0185 |                            14.8649 | 0.17s  |

<!-- END mdsh -->

</details>

### Hard

This experiment uses 10 arms. All arms have a success probability of 0.5, except
for the best arm, which has a success probability of 0.51.

This experiment was taken from the paper describing Boltzmann-Gumbel Exploration.

<details>
<summary>Results</summary>

<!-- `> cat experiments/hard.md` -->

<!-- BEGIN mdsh -->

| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| POKER (H=100)                                               |     18.10 |        4.0949 |                             0.1700 | 0.36s  |
| Greedy                                                      |     17.00 |        4.1498 |                             0.1100 | 0.08s  |
| POKER (H=10)                                                |     17.00 |        4.1498 |                             0.1100 | 0.40s  |
| POKER (H=1)                                                 |     17.00 |        4.1498 |                             0.1100 | 0.35s  |
| POKER (H=25)                                                |     17.00 |        4.1498 |                             0.1100 | 0.33s  |
| POKER (H=5)                                                 |     17.00 |        4.1498 |                             0.1100 | 0.40s  |
| POKER (H=50)                                                |     17.00 |        4.1499 |                             0.1100 | 0.34s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     16.90 |        4.1552 |                             0.1000 | 0.16s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     16.80 |        4.1598 |                             0.1000 | 0.16s  |
| ϵ-Greedy (ϵ=0.010)                                          |     16.64 |        4.1682 |                             0.1000 | 0.09s  |
| ReUCB (a=1.00)                                              |     16.59 |        4.1704 |                             0.1200 | 0.94s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     16.29 |        4.1854 |                             0.1000 | 0.17s  |
| ϵ-Greedy (ϵ=0.020)                                          |     16.25 |        4.1873 |                             0.1000 | 0.12s  |
| ReUCB (a=1.50)                                              |     15.54 |        4.2231 |                             0.1200 | 0.97s  |
| ReUCB (a=2.00)                                              |     15.41 |        4.2293 |                             0.1200 | 0.98s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     15.26 |        4.2371 |                             0.1700 | 0.15s  |
| Gittins Index -- Whittle's Approximation (β=0.50)           |     15.19 |        4.2406 |                             0.1200 | 0.19s  |
| ϵ-Greedy (ϵ=0.050)                                          |     15.11 |        4.2447 |                             0.0900 | 0.12s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     14.77 |        4.2614 |                             0.0800 | 0.15s  |
| VarTS                                                       |     14.62 |        4.2691 |                             0.1200 | 0.46s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     14.23 |        4.2887 |                             0.1700 | 0.75s  |
| Batch Ensemble for MAB (m=8)                                |     14.22 |        4.2892 |                             0.3800 | 0.33s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     14.10 |        4.2951 |                             0.1700 | 7.10s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     14.05 |        4.2973 |                             0.1600 | 0.14s  |
| ϵ-Greedy (ϵ=0.100)                                          |     13.97 |        4.3014 |                             0.0800 | 0.11s  |
| ϵ-Exploring Thompson Sampling                               |     13.74 |        4.3130 |                             0.1100 | 0.14s  |
| Batch Ensemble for MAB (m=2)                                |     13.68 |        4.3160 |                             0.2900 | 0.17s  |
| Forced Exploration                                          |     13.53 |        4.3235 |                             0.1000 | 0.07s  |
| RS (a=0.50)                                                 |     13.42 |        4.3290 |                             0.0100 | 0.20s  |
| IRS.FH (H=1)                                                |     13.36 |        4.3318 |                             0.1200 | 1.18s  |
| UCB-DT (γ=0.90)                                             |     13.27 |        4.3365 |                             0.1000 | 2.73s  |
| UCB-DT (γ=0.95)                                             |     13.27 |        4.3365 |                             0.1000 | 2.74s  |
| Gittins Index -- Whittle's Approximation (β=0.70)           |     13.25 |        4.3374 |                             0.1200 | 0.22s  |
| UCB-DT (γ=1.00)                                             |     13.19 |        4.3406 |                             0.1200 | 2.71s  |
| SoftElim (w=0.10)                                           |     13.13 |        4.3434 |                             0.1100 | 0.37s  |
| UCB-DT (γ=0.75)                                             |     13.05 |        4.3474 |                             0.1000 | 2.76s  |
| MOSS-Anytime (α=-0.33)                                      |     13.00 |        4.3502 |                             0.2000 | 0.21s  |
| MOSS-Anytime (α=-0.85)                                      |     12.95 |        4.3526 |                             0.1800 | 0.18s  |
| MOSS-Anytime (α=-0.50)                                      |     12.94 |        4.3532 |                             0.1700 | 0.21s  |
| IRS.FH (H=2)                                                |     12.84 |        4.3582 |                             0.1700 | 1.40s  |
| Batch Ensemble for MAB (m=1)                                |     12.71 |        4.3646 |                             0.4800 | 0.09s  |
| Gittins Index -- Whittle's Approximation (β=0.99)           |     12.66 |        4.3672 |                             0.2000 | 0.31s  |
| IRS.FH (H=3)                                                |     12.63 |        4.3687 |                             0.1900 | 1.61s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.5)                          |     12.62 |        4.3690 |                             0.1800 | 0.23s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.1)                          |     12.62 |        4.3692 |                             0.2100 | 0.23s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.001)                       |     12.58 |        4.3710 |                             0.1800 | 0.21s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.1)                         |     12.58 |        4.3710 |                             0.1800 | 0.25s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.5)                         |     12.58 |        4.3710 |                             0.1800 | 0.26s  |
| Batch Ensemble for MAB (m=0)                                |     12.57 |        4.3714 |                             0.1700 | 0.05s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.8)     |     12.47 |        4.3767 |                             0.1200 | 0.35s  |
| BayesUCB (δ=0.500)                                          |     12.44 |        4.3782 |                             0.2000 | 0.20s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.001)                        |     12.41 |        4.3797 |                             0.2600 | 0.23s  |
| POKER (H=250)                                               |     12.40 |        4.3802 |                             0.2500 | 0.36s  |
| BayesUCB (δ=0.400)                                          |     12.37 |        4.3813 |                             0.2000 | 0.20s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.9)     |     12.37 |        4.3816 |                             0.1200 | 0.40s  |
| Gittins Index -- Whittle's Approximation (β=0.90)           |     12.37 |        4.3816 |                             0.1700 | 0.23s  |
| UCB-DT (γ=0.25)                                             |     12.33 |        4.3836 |                             0.1500 | 2.76s  |
| IRS.FH (H=4)                                                |     12.29 |        4.3857 |                             0.1900 | 1.55s  |
| TS-UCB (100 samples)                                        |     12.17 |        4.3915 |                             0.2500 | 72.48s |
| UCBT                                                        |     12.17 |        4.3916 |                             0.4200 | 0.07s  |
| BayesUCB (δ=0.300)                                          |     12.17 |        4.3917 |                             0.2500 | 0.20s  |
| IRS.FH (H=5)                                                |     12.02 |        4.3990 |                             0.2200 | 1.49s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.001)                           |     12.01 |        4.3996 |                             0.3500 | 0.28s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.5)                             |     12.00 |        4.3999 |                             0.2500 | 0.24s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.1)                             |     11.98 |        4.4008 |                             0.2500 | 0.23s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     11.91 |        4.4043 |                             0.1500 | 0.08s  |
| Bootstrapped Thompson Sampling (J=10)                       |     11.83 |        4.4083 |                             0.1600 | 0.40s  |
| Bootstrapped Thompson Sampling (J=500)                      |     11.80 |        4.4101 |                             0.3400 | 4.35s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     11.78 |        4.4109 |                             0.3400 | 8.47s  |
| Bootstrapped Thompson Sampling (J=100)                      |     11.76 |        4.4118 |                             0.3100 | 1.08s  |
| UCB-DT (γ=0.50)                                             |     11.76 |        4.4119 |                             0.0900 | 2.75s  |
| EB-TCI                                                      |     11.56 |        4.4218 |                             0.4400 | 0.31s  |
| Batch Ensemble for MAB (m=4)                                |     11.56 |        4.4221 |                             0.3100 | 0.27s  |
| WR-SDA (forced_exploration=false)                           |     11.52 |        4.4238 |                             0.3200 | 1.35s  |
| BayesUCB (δ=0.200)                                          |     11.51 |        4.4245 |                             0.2300 | 0.19s  |
| IRS.FH (H=10)                                               |     11.46 |        4.4270 |                             0.2400 | 1.51s  |
| TS-UCB (10 samples)                                         |     11.45 |        4.4275 |                             0.2500 | 7.37s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     11.45 |        4.4276 |                             0.2600 | 5.01s  |
| Vanilla Residual Bootstrap (init=0)                         |     11.42 |        4.4292 |                             0.3500 | 0.19s  |
| MARS (δ=0.100)                                              |     11.41 |        4.4296 |                             0.2400 | 0.32s  |
| WR-SDA (forced_exploration=true)                            |     11.41 |        4.4296 |                             0.3000 | 1.52s  |
| CODE (δ=0.900)                                              |     11.39 |        4.4305 |                             0.4900 | 0.18s  |
| ReBoot (r=0.25)                                             |     11.38 |        4.4311 |                             0.3500 | 0.20s  |
| BayesUCB (δ=0.900)                                          |     11.37 |        4.4314 |                             0.1200 | 0.20s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.95)    |     11.35 |        4.4324 |                             0.1200 | 0.41s  |
| ReBoot (r=0.50)                                             |     11.34 |        4.4329 |                             0.3800 | 0.24s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.99)    |     11.29 |        4.4355 |                             0.0900 | 0.42s  |
| TS-UCB (1 samples)                                          |     11.21 |        4.4395 |                             0.2400 | 0.95s  |
| CODE (δ=0.990)                                              |     11.21 |        4.4397 |                             0.1200 | 0.16s  |
| Garbage In, Reward Out (a=0.10)                             |     11.16 |        4.4418 |                             0.3400 | 1.01s  |
| BayesUCB (δ=0.100)                                          |     11.16 |        4.4421 |                             0.2400 | 0.17s  |
| Non-Parametric Thompson Sampling                            |     11.16 |        4.4422 |                             0.3400 | 4.08s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     11.15 |        4.4425 |                             0.3400 | 1.00s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     11.15 |        4.4426 |                             0.3300 | 0.90s  |
| Thompson Sampling                                           |     11.14 |        4.4429 |                             0.3300 | 0.65s  |
| IRS.FH (H=25)                                               |     11.14 |        4.4431 |                             0.2600 | 2.09s  |
| SoftElim (w=0.50)                                           |     11.14 |        4.4431 |                             0.2500 | 0.40s  |
| Perturbed-History Exploration (a=1.1)                       |     11.13 |        4.4433 |                             0.3600 | 0.84s  |
| Weighted Bootstrap                                          |     11.13 |        4.4436 |                             0.3400 | 3.11s  |
| Multiplier Bootstrap-based Exploration                      |     11.12 |        4.4439 |                             0.3100 | 6.04s  |
| Softsatisficing (a=0.65)                                    |     11.12 |        4.4441 |                             0.3000 | 0.38s  |
| Vanilla Residual Bootstrap (init=1)                         |     11.11 |        4.4443 |                             0.3500 | 0.24s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     11.09 |        4.4454 |                             0.4000 | 1.10s  |
| Garbage In, Reward Out (a=0.33)                             |     11.04 |        4.4480 |                             0.3800 | 1.18s  |
| Tsallis-INF                                                 |     11.01 |        4.4497 |                             0.2700 | 1.10s  |
| UCB-DT (γ=0.10)                                             |     11.00 |        4.4502 |                             0.1200 | 2.84s  |
| KL-UCB                                                      |     10.99 |        4.4505 |                             0.2800 | 8.11s  |
| MARS (δ=1.000)                                              |     10.95 |        4.4523 |                             0.0000 | 0.09s  |
| RS (a=0.65)                                                 |     10.95 |        4.4523 |                             0.2500 | 0.17s  |
| ReBoot (r=0.90)                                             |     10.94 |        4.4528 |                             0.3800 | 0.24s  |
| MARS (δ=0.010)                                              |     10.93 |        4.4537 |                             0.3700 | 1.39s  |
| Kullback-Leibler Maillard Sampling                          |     10.91 |        4.4544 |                             0.3500 | 0.63s  |
| Perturbed-History Exploration (a=2.1)                       |     10.89 |        4.4557 |                             0.3400 | 1.02s  |
| SoftElim (w=1.00)                                           |     10.88 |        4.4560 |                             0.3300 | 0.41s  |
| Vanilla Residual Bootstrap (init=5)                         |     10.85 |        4.4574 |                             0.2700 | 0.24s  |
| Hellinger-UCB                                               |     10.85 |        4.4575 |                             0.2800 | 2.46s  |
| lil' UCB (δ=0.100)                                          |     10.85 |        4.4575 |                             0.2600 | 0.29s  |
| ReBoot (r=1.00)                                             |     10.84 |        4.4578 |                             0.3500 | 0.24s  |
| Bounded Dirichlet Sampling                                  |     10.83 |        4.4586 |                             0.3100 | 2.04s  |
| MARS (δ=0.002)                                              |     10.79 |        4.4603 |                             0.2800 | 5.94s  |
| UCB1-Tuned                                                  |     10.74 |        4.4632 |                             0.2400 | 0.16s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     10.72 |        4.4641 |                             0.3100 | 1.16s  |
| MARS (δ=0.001)                                              |     10.71 |        4.4646 |                             0.3200 | 11.39s |
| lil' UCB (δ=0.010)                                          |     10.70 |        4.4651 |                             0.2200 | 0.29s  |
| Boltzmann-Gumbel Exploration                                |     10.67 |        4.4663 |                             0.2700 | 0.31s  |
| Garbage In, Reward Out (a=1.00)                             |     10.66 |        4.4669 |                             0.2600 | 1.33s  |
| Gradient Bandit                                             |     10.66 |        4.4672 |                             0.2000 | 0.38s  |
| Gradient Bandit (with baseline)                             |     10.60 |        4.4699 |                             0.2000 | 0.45s  |
| FTPL-GR (lr=0.100)                                          |     10.58 |        4.4711 |                             0.2500 | 4.39s  |
| lil' UCB (δ=0.001)                                          |     10.54 |        4.4730 |                             0.2000 | 0.26s  |
| ReBoot (r=1.50)                                             |     10.49 |        4.4756 |                             0.2100 | 0.25s  |
| FTPL-GR (lr=1.000)                                          |     10.46 |        4.4769 |                             0.2900 | 4.05s  |
| Perturbed-History Exploration (a=5.1)                       |     10.40 |        4.4798 |                             0.1900 | 1.01s  |
| ReBoot (r=1.70)                                             |     10.40 |        4.4798 |                             0.1800 | 0.26s  |
| Softsatisficing (a=0.75)                                    |     10.38 |        4.4810 |                             0.1900 | 0.44s  |
| RS (a=0.75)                                                 |     10.36 |        4.4819 |                             0.1600 | 0.20s  |
| EXP-IX                                                      |     10.36 |        4.4822 |                             0.1600 | 0.42s  |
| ReBoot (r=2.10)                                             |     10.29 |        4.4854 |                             0.1400 | 0.24s  |
| RS (a=0.00)                                                 |     10.28 |        4.4861 |                             0.0000 | 0.13s  |
| ETC (m=25)                                                  |     10.27 |        4.4863 |                             0.0000 | 0.16s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.1)                             |     10.25 |        4.4873 |                             0.1100 | 0.24s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.5)                             |     10.25 |        4.4873 |                             0.1100 | 0.23s  |
| RS (a=0.10)                                                 |     10.24 |        4.4881 |                             0.0000 | 0.16s  |
| UCB1                                                        |     10.23 |        4.4883 |                             0.1600 | 0.13s  |
| Softsatisficing (a=0.50)                                    |     10.23 |        4.4884 |                             0.0000 | 0.09s  |
| RS (a=0.90)                                                 |     10.22 |        4.4891 |                             0.1100 | 0.17s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.001)                           |     10.21 |        4.4893 |                             0.1200 | 0.24s  |
| Softsatisficing (a=0.90)                                    |     10.21 |        4.4897 |                             0.1200 | 0.42s  |
| SoftElim (w=2.00)                                           |     10.20 |        4.4898 |                             0.1200 | 0.40s  |
| Softsatisficing (a=0.99)                                    |     10.17 |        4.4914 |                             0.1100 | 0.44s  |
| FTPL-GR (lr=0.010)                                          |     10.14 |        4.4930 |                             0.0900 | 4.48s  |
| ETC (m=5)                                                   |     10.11 |        4.4943 |                             0.0000 | 0.15s  |
| ETC (m=20)                                                  |     10.11 |        4.4946 |                             0.0000 | 0.15s  |
| SoftElim (w=2.83)                                           |     10.10 |        4.4949 |                             0.0800 | 0.41s  |
| Least Failures                                              |     10.08 |        4.4961 |                             0.0900 | 0.06s  |
| RS (a=0.99)                                                 |     10.08 |        4.4961 |                             0.0900 | 0.16s  |
| UCB-DT (γ=0.02)                                             |     10.06 |        4.4969 |                             0.0900 | 2.35s  |
| Softsatisficing (a=0.25)                                    |     10.05 |        4.4976 |                             0.0000 | 0.07s  |
| ETC (m=2)                                                   |     10.04 |        4.4982 |                             0.4300 | 0.11s  |
| ETC (m=3)                                                   |     10.04 |        4.4982 |                             0.4300 | 0.15s  |
| FTPL-GR (lr=0.001)                                          |     10.02 |        4.4988 |                             0.0400 | 4.31s  |
| Random                                                      |     10.02 |        4.4992 |                             0.0500 | 0.02s  |
| CODE (δ=0.050)                                              |     10.00 |        4.5000 |                             0.0000 | 0.17s  |
| RS (a=0.25)                                                 |     10.00 |        4.5000 |                             0.0000 | 0.18s  |
| Softsatisficing (a=0.10)                                    |     10.00 |        4.5000 |                             0.0000 | 0.06s  |
| ETC (m=10)                                                  |      9.94 |        4.5030 |                             0.0000 | 0.15s  |

<!-- END mdsh -->

</details>

### Beta

This experiment uses 10 arms. The arm means are sampled from a Beta(1, 8) distribution.

This experiment was taken from the paper _Multiplier Bootstrap-based Exploration_.

<details>
<summary>Results</summary>

<!-- `> cat experiments/beta.md` -->

<!-- BEGIN mdsh -->

| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| BayesUCB (δ=0.900)                                          |     57.80 |       20.8173 |                             5.4766 | 0.20s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.95)    |     57.01 |       21.0752 |                             5.5688 | 0.42s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.9)     |     56.78 |       21.0892 |                             5.6266 | 0.41s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.1)                         |     55.98 |       21.1003 |                             6.6151 | 0.26s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.5)                         |     55.97 |       21.1006 |                             6.6257 | 0.26s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.8)     |     56.64 |       21.1021 |                             5.7940 | 0.37s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.001)                       |     55.97 |       21.1268 |                             6.5255 | 0.22s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.1)                          |     55.99 |       21.1428 |                             6.5261 | 0.26s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.5)                          |     55.95 |       21.1445 |                             6.5317 | 0.26s  |
| Gittins Index -- Whittle's Approximation (β=0.70)           |     57.25 |       21.1454 |                             5.2421 | 0.27s  |
| Gittins Index -- Whittle's Approximation (β=0.50)           |     56.58 |       21.2012 |                             5.5556 | 0.20s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.99)    |     57.41 |       21.2745 |                             4.9635 | 0.48s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.001)                        |     55.91 |       21.2843 |                             6.6230 | 0.32s  |
| Gittins Index -- Whittle's Approximation (β=0.90)           |     57.73 |       21.7090 |                             4.9167 | 0.23s  |
| UCB-DT (γ=0.50)                                             |     56.40 |       21.7948 |                             5.7423 | 2.55s  |
| IRS.FH (H=1)                                                |     55.74 |       21.8777 |                             5.8971 | 1.23s  |
| MOSS-Anytime (α=-0.85)                                      |     54.95 |       22.2898 |                             5.6517 | 0.19s  |
| Batch Ensemble for MAB (m=0)                                |     57.67 |       22.3441 |                             5.1453 | 0.06s  |
| UCB-DT (γ=0.75)                                             |     54.64 |       22.4071 |                             6.1492 | 2.55s  |
| UCB-DT (γ=0.90)                                             |     54.45 |       22.4627 |                             6.1571 | 2.58s  |
| UCB-DT (γ=0.95)                                             |     54.39 |       22.4968 |                             6.1852 | 2.58s  |
| UCB-DT (γ=1.00)                                             |     53.32 |       22.6778 |                             7.3649 | 2.48s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     56.94 |       22.9408 |                             7.1147 | 17.26s |
| IRS.FH (H=2)                                                |     56.33 |       23.0910 |                             5.0660 | 1.25s  |
| CODE (δ=0.990)                                              |     51.11 |       23.5974 |                             9.3932 | 0.17s  |
| Gittins Index -- Whittle's Approximation (β=0.99)           |     56.84 |       23.7789 |                             5.0319 | 0.22s  |
| MOSS-Anytime (α=-0.50)                                      |     56.24 |       24.1465 |                             4.0881 | 0.22s  |
| BayesUCB (δ=0.500)                                          |     56.42 |       24.2684 |                             5.5883 | 0.20s  |
| IRS.FH (H=3)                                                |     55.61 |       24.5294 |                             4.7675 | 1.28s  |
| TS-UCB (100 samples)                                        |     56.12 |       24.7396 |                             4.2715 | 69.47s |
| ReBoot (r=0.25)                                             |     52.26 |       24.7586 |                             8.6759 | 0.21s  |
| UCB-DT (γ=0.25)                                             |     55.33 |       24.8894 |                             6.0260 | 2.58s  |
| IRS.FH (H=4)                                                |     55.19 |       25.4645 |                             4.9019 | 1.32s  |
| BayesUCB (δ=0.400)                                          |     55.31 |       25.7435 |                             5.7135 | 0.20s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.5)                             |     54.87 |       26.1006 |                             5.6929 | 0.26s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.1)                             |     54.85 |       26.1238 |                             5.6912 | 0.26s  |
| MOSS-Anytime (α=-0.33)                                      |     54.80 |       26.1464 |                             4.2098 | 0.22s  |
| IRS.FH (H=5)                                                |     54.72 |       26.2315 |                             4.9551 | 1.34s  |
| TS-UCB (10 samples)                                         |     55.00 |       26.2465 |                             4.2202 | 7.47s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.001)                           |     54.65 |       26.3698 |                             5.5242 | 0.28s  |
| SoftElim (w=0.10)                                           |     47.37 |       26.4249 |                             8.8470 | 0.37s  |
| MARS (δ=0.100)                                              |     50.54 |       26.5911 |                             7.3123 | 0.32s  |
| Batch Ensemble for MAB (m=1)                                |     53.77 |       26.8886 |                             7.9552 | 0.10s  |
| BayesUCB (δ=0.300)                                          |     53.93 |       27.5129 |                             5.7981 | 0.20s  |
| RS (a=0.25)                                                 |     51.31 |       27.6540 |                            15.4650 | 0.19s  |
| IRS.FH (H=10)                                               |     53.19 |       28.6872 |                             5.2676 | 1.45s  |
| SoftElim (w=0.50)                                           |     51.91 |       28.7964 |                             6.3713 | 0.40s  |
| UCBT                                                        |     47.49 |       28.8558 |                             8.0049 | 0.08s  |
| ReBoot (r=0.50)                                             |     51.44 |       28.9633 |                             6.3791 | 0.23s  |
| MARS (δ=0.010)                                              |     51.25 |       29.2541 |                             7.4360 | 1.37s  |
| TS-UCB (1 samples)                                          |     52.69 |       29.2908 |                             4.9082 | 0.94s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     47.60 |       29.4661 |                             9.1640 | 0.24s  |
| Softsatisficing (a=0.25)                                    |     49.07 |       29.5243 |                            16.9209 | 0.29s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     47.35 |       29.8509 |                             9.2614 | 1.08s  |
| BayesUCB (δ=0.200)                                          |     51.93 |       29.8533 |                             5.8074 | 0.20s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     47.30 |       29.8928 |                             9.1374 | 9.35s  |
| RS (a=0.10)                                                 |     44.30 |       29.9723 |                            13.9283 | 0.17s  |
| Hellinger-UCB                                               |     50.41 |       30.1850 |                             5.4750 | 2.74s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     45.66 |       30.9426 |                            10.3885 | 0.14s  |
| Bootstrapped Thompson Sampling (J=10)                       |     49.88 |       31.1623 |                             6.5576 | 0.38s  |
| Forced Exploration                                          |     48.86 |       31.4112 |                             9.0715 | 0.10s  |
| WR-SDA (forced_exploration=true)                            |     45.51 |       31.4285 |                             8.7057 | 2.14s  |
| IRS.FH (H=25)                                               |     50.57 |       32.0301 |                             5.6474 | 1.61s  |
| Multiplier Bootstrap-based Exploration                      |     49.17 |       32.4139 |                             6.0942 | 6.08s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     42.88 |       32.7340 |                            12.0469 | 0.15s  |
| ϵ-Exploring Thompson Sampling                               |     44.38 |       33.2239 |                            12.5400 | 0.16s  |
| ϵ-Greedy (ϵ=0.100)                                          |     44.10 |       33.2831 |                            11.8153 | 0.15s  |
| BayesUCB (δ=0.100)                                          |     48.69 |       33.6207 |                             5.8840 | 0.17s  |
| ϵ-Greedy (ϵ=0.050)                                          |     42.23 |       33.7998 |                            13.3609 | 0.17s  |
| UCB1-Tuned                                                  |     48.22 |       34.0173 |                             5.5690 | 0.18s  |
| Bootstrapped Thompson Sampling (J=100)                      |     47.17 |       34.7044 |                             6.3147 | 1.05s  |
| Garbage In, Reward Out (a=0.10)                             |     46.53 |       35.1309 |                             6.4203 | 0.79s  |
| Bootstrapped Thompson Sampling (J=500)                      |     46.87 |       35.1931 |                             6.3344 | 4.79s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     46.77 |       35.2492 |                             6.3528 | 9.92s  |
| Vanilla Residual Bootstrap (init=1)                         |     46.87 |       35.3194 |                             6.1483 | 0.20s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     40.31 |       35.3717 |                            14.8777 | 0.15s  |
| ϵ-Greedy (ϵ=0.020)                                          |     39.94 |       35.9324 |                            16.4079 | 0.17s  |
| Softsatisficing (a=0.10)                                    |     39.16 |       36.0339 |                            20.3466 | 0.07s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     39.46 |       36.2891 |                            16.3213 | 0.16s  |
| ReUCB (a=2.00)                                              |     39.22 |       36.5714 |                            16.0590 | 1.00s  |
| ReUCB (a=1.50)                                              |     39.18 |       36.6102 |                            16.1550 | 0.97s  |
| Vanilla Residual Bootstrap (init=0)                         |     39.16 |       36.6560 |                            16.2068 | 0.17s  |
| ReUCB (a=1.00)                                              |     39.11 |       36.6897 |                            16.2417 | 0.96s  |
| UCB-DT (γ=0.10)                                             |     47.09 |       36.9910 |                             7.3436 | 2.57s  |
| ETC (m=5)                                                   |     39.97 |       37.5465 |                            17.0296 | 0.15s  |
| MARS (δ=0.002)                                              |     41.98 |       37.6275 |                            11.3986 | 5.07s  |
| Weighted Bootstrap                                          |     45.12 |       37.7328 |                             6.5247 | 3.09s  |
| Thompson Sampling                                           |     45.10 |       37.7381 |                             6.5241 | 0.64s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     45.09 |       37.7390 |                             6.4866 | 1.10s  |
| ϵ-Greedy (ϵ=0.010)                                          |     38.41 |       37.7394 |                            18.4671 | 0.12s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     44.99 |       37.8316 |                             6.5136 | 0.99s  |
| KL-UCB                                                      |     44.75 |       37.9754 |                             5.7666 | 7.98s  |
| MARS (δ=0.001)                                              |     39.54 |       38.9137 |                            13.8711 | 9.18s  |
| ETC (m=10)                                                  |     40.33 |       38.9869 |                            13.6763 | 0.15s  |
| Non-Parametric Thompson Sampling                            |     43.90 |       39.3507 |                             6.7160 | 4.12s  |
| Greedy                                                      |     36.66 |       39.9099 |                            21.7087 | 0.09s  |
| Bounded Dirichlet Sampling                                  |     43.54 |       39.9645 |                             6.6454 | 2.55s  |
| CODE (δ=0.900)                                              |     40.61 |       40.2050 |                            13.2482 | 0.19s  |
| ReBoot (r=0.90)                                             |     42.54 |       40.7881 |                             7.1968 | 0.23s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     41.59 |       40.8984 |                             7.1496 | 1.04s  |
| WR-SDA (forced_exploration=false)                           |     36.73 |       41.0443 |                            19.6162 | 1.86s  |
| POKER (H=100)                                               |     36.39 |       41.3770 |                            22.1022 | 0.26s  |
| POKER (H=250)                                               |     36.40 |       41.3832 |                            21.9572 | 0.26s  |
| POKER (H=50)                                                |     36.31 |       41.4064 |                            22.3819 | 0.27s  |
| POKER (H=25)                                                |     36.24 |       41.4405 |                            22.6058 | 0.26s  |
| Kullback-Leibler Maillard Sampling                          |     40.83 |       41.4463 |                             7.5405 | 0.49s  |
| POKER (H=10)                                                |     36.05 |       41.5399 |                            23.0560 | 0.26s  |
| POKER (H=5)                                                 |     35.92 |       41.6486 |                            23.3861 | 0.26s  |
| POKER (H=1)                                                 |     35.72 |       41.8819 |                            23.9184 | 0.23s  |
| VarTS                                                       |     44.32 |       41.9995 |                             7.9769 | 0.47s  |
| Perturbed-History Exploration (a=1.1)                       |     40.79 |       42.7866 |                             7.3646 | 0.89s  |
| ReBoot (r=1.00)                                             |     40.65 |       43.3432 |                             7.6618 | 0.23s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     41.58 |       44.0842 |                            11.9547 | 0.12s  |
| Garbage In, Reward Out (a=0.33)                             |     38.75 |       45.1922 |                             7.8091 | 1.05s  |
| ETC (m=3)                                                   |     33.51 |       45.7840 |                            28.1017 | 0.15s  |
| ETC (m=20)                                                  |     37.94 |       47.1134 |                            13.4466 | 0.15s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     33.63 |       48.2505 |                             9.6052 | 1.12s  |
| lil' UCB (δ=0.100)                                          |     36.34 |       48.6046 |                             7.3285 | 0.30s  |
| Batch Ensemble for MAB (m=2)                                |     35.22 |       48.8043 |                             9.6369 | 0.20s  |
| ETC (m=25)                                                  |     37.82 |       51.7141 |                            13.9357 | 0.16s  |
| SoftElim (w=1.00)                                           |     33.15 |       51.8623 |                             8.8820 | 0.40s  |
| ETC (m=2)                                                   |     29.53 |       53.1694 |                            28.6333 | 0.14s  |
| ReBoot (r=1.50)                                             |     33.20 |       53.6329 |                            10.8113 | 0.23s  |
| Perturbed-History Exploration (a=2.1)                       |     32.69 |       53.7141 |                             9.5801 | 1.06s  |
| Tsallis-INF                                                 |     32.39 |       54.7917 |                            11.3371 | 1.25s  |
| Gradient Bandit                                             |     33.08 |       54.8612 |                            11.0784 | 0.37s  |
| Gradient Bandit (with baseline)                             |     32.39 |       55.7285 |                            11.4145 | 0.43s  |
| Vanilla Residual Bootstrap (init=5)                         |     31.29 |       56.3702 |                            10.3022 | 0.21s  |
| ReBoot (r=1.70)                                             |     30.99 |       56.8129 |                            12.0371 | 0.24s  |
| Garbage In, Reward Out (a=1.00)                             |     29.48 |       58.3347 |                            11.5697 | 1.17s  |
| Boltzmann-Gumbel Exploration                                |     29.89 |       58.4917 |                            11.5794 | 0.34s  |
| lil' UCB (δ=0.010)                                          |     29.25 |       58.7242 |                            11.2953 | 0.29s  |
| EB-TCI                                                      |     24.42 |       59.0388 |                            22.7179 | 0.28s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     30.85 |       61.5675 |                            16.7498 | 0.08s  |
| ReBoot (r=2.10)                                             |     27.57 |       61.9376 |                            14.1671 | 0.30s  |
| Batch Ensemble for MAB (m=4)                                |     26.13 |       62.3496 |                            14.4520 | 0.35s  |
| RS (a=0.00)                                                 |     21.30 |       64.4828 |                            45.7779 | 0.12s  |
| lil' UCB (δ=0.001)                                          |     25.41 |       64.5631 |                            14.7753 | 0.28s  |
| FTPL-GR (lr=0.100)                                          |     24.27 |       65.5709 |                            16.0043 | 4.04s  |
| Batch Ensemble for MAB (m=8)                                |     21.62 |       67.3453 |                            17.1377 | 0.52s  |
| MARS (δ=1.000)                                              |     21.56 |       67.5569 |                            47.1317 | 0.08s  |
| Perturbed-History Exploration (a=5.1)                       |     23.50 |       67.6540 |                            15.6673 | 1.10s  |
| RS (a=0.50)                                                 |     22.27 |       69.0806 |                            16.5670 | 0.22s  |
| Softsatisficing (a=0.50)                                    |     22.04 |       69.2504 |                            16.5095 | 0.54s  |
| UCB1                                                        |     22.31 |       69.6096 |                            17.0817 | 0.16s  |
| FTPL-GR (lr=1.000)                                          |     18.50 |       75.4350 |                            20.0029 | 4.55s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.001)                           |     18.85 |       75.5057 |                            19.3704 | 0.27s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.5)                             |     18.84 |       75.5100 |                            19.3704 | 0.28s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.1)                             |     18.84 |       75.5124 |                            19.3704 | 0.27s  |
| SoftElim (w=2.00)                                           |     17.86 |       76.0577 |                            18.9139 | 0.40s  |
| EXP-IX                                                      |     17.71 |       77.5879 |                            20.1208 | 0.49s  |
| Softsatisficing (a=0.65)                                    |     16.04 |       80.3949 |                            20.9280 | 0.53s  |
| RS (a=0.65)                                                 |     16.07 |       80.4757 |                            21.2548 | 0.19s  |
| UCB-DT (γ=0.02)                                             |     15.85 |       81.2599 |                            20.7454 | 2.32s  |
| SoftElim (w=2.83)                                           |     14.01 |       83.6221 |                            22.1874 | 0.40s  |
| FTPL-GR (lr=0.010)                                          |     14.62 |       83.6299 |                            22.2985 | 4.38s  |
| RS (a=0.75)                                                 |     14.49 |       83.6745 |                            22.3955 | 0.20s  |
| Softsatisficing (a=0.75)                                    |     14.41 |       83.6750 |                            22.5529 | 0.44s  |
| RS (a=0.90)                                                 |     13.27 |       86.1578 |                            23.4025 | 0.19s  |
| Softsatisficing (a=0.90)                                    |     13.19 |       86.1783 |                            23.4203 | 0.44s  |
| Least Failures                                              |     12.90 |       86.8718 |                            23.7396 | 0.19s  |
| RS (a=0.99)                                                 |     12.90 |       86.8720 |                            23.7404 | 0.19s  |
| Softsatisficing (a=0.99)                                    |     12.75 |       87.0894 |                            23.7866 | 0.46s  |
| FTPL-GR (lr=0.001)                                          |     10.39 |       92.2580 |                            25.9058 | 4.13s  |
| Random                                                      |      9.99 |       93.1436 |                            26.0904 | 0.03s  |
| CODE (δ=0.050)                                              |     10.00 |       93.1468 |                            25.9588 | 0.18s  |

<!-- END mdsh -->

</details>

### Reverse Beta

This experiment uses 10 arms. The arm means are sampled from a Beta(8, 1) distribution.

I added this to see which algorithms are affected by rewards close to 1 instead of close to 0.

<details>
<summary>Results</summary>

<!-- `> cat experiments/reverse_beta.md` -->

<!-- BEGIN mdsh -->

| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| TS-UCB (100 samples)                                        |     57.88 |        6.9424 |                             2.3169 | 66.31s |
| TS-UCB (10 samples)                                         |     57.81 |        7.2576 |                             2.1382 | 6.23s  |
| RS (a=0.90)                                                 |     44.63 |        7.3959 |                             3.7683 | 0.16s  |
| POKER (H=5)                                                 |     55.21 |        7.8042 |                             1.8429 | 0.34s  |
| TS-UCB (1 samples)                                          |     57.78 |        7.8597 |                             1.9439 | 0.79s  |
| Batch Ensemble for MAB (m=0)                                |     56.24 |        7.8614 |                             1.7577 | 0.06s  |
| POKER (H=1)                                                 |     56.13 |        8.0894 |                             1.7428 | 0.38s  |
| POKER (H=10)                                                |     48.12 |        8.3258 |                             2.2912 | 0.39s  |
| UCB-DT (γ=1.00)                                             |     55.66 |        8.4709 |                             1.4837 | 2.71s  |
| IRS.FH (H=2)                                                |     54.85 |        8.4788 |                             1.5053 | 1.39s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     54.56 |        8.4801 |                             1.5113 | 9.15s  |
| IRS.FH (H=3)                                                |     55.12 |        8.4888 |                             1.5141 | 1.47s  |
| SoftElim (w=0.10)                                           |     54.88 |        8.5051 |                             1.4831 | 0.36s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     54.54 |        8.5219 |                             1.5144 | 1.01s  |
| IRS.FH (H=1)                                                |     54.33 |        8.5340 |                             1.5095 | 1.26s  |
| IRS.FH (H=4)                                                |     55.12 |        8.5479 |                             1.5428 | 1.35s  |
| UCB-DT (γ=0.90)                                             |     55.84 |        8.5612 |                             1.5143 | 2.71s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     54.46 |        8.5696 |                             1.5205 | 0.18s  |
| UCB-DT (γ=0.95)                                             |     55.76 |        8.5784 |                             1.5121 | 2.72s  |
| IRS.FH (H=5)                                                |     55.41 |        8.5928 |                             1.5664 | 1.47s  |
| ReUCB (a=1.50)                                              |     54.41 |        8.5966 |                             1.5037 | 0.95s  |
| ReUCB (a=2.00)                                              |     54.44 |        8.5966 |                             1.5062 | 0.96s  |
| ReUCB (a=1.00)                                              |     54.34 |        8.6079 |                             1.5056 | 0.92s  |
| Greedy                                                      |     53.82 |        8.6471 |                             1.5273 | 0.08s  |
| UCB-DT (γ=0.75)                                             |     55.91 |        8.6739 |                             1.5558 | 2.72s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     53.57 |        8.8897 |                             1.5610 | 0.15s  |
| Batch Ensemble for MAB (m=1)                                |     51.73 |        8.9409 |                             4.0595 | 0.08s  |
| IRS.FH (H=10)                                               |     55.40 |        8.9571 |                             1.7240 | 1.53s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     53.45 |        9.0268 |                             1.5815 | 0.15s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.99)    |     39.47 |        9.0345 |                             4.6410 | 0.48s  |
| BayesUCB (δ=0.200)                                          |     43.36 |        9.0488 |                             4.8228 | 0.19s  |
| ϵ-Greedy (ϵ=0.010)                                          |     53.34 |        9.0561 |                             1.5947 | 0.09s  |
| UCB-DT (γ=0.50)                                             |     55.53 |        9.0760 |                             1.7753 | 2.72s  |
| ϵ-Greedy (ϵ=0.020)                                          |     52.84 |        9.4814 |                             1.6900 | 0.12s  |
| CODE (δ=0.990)                                              |     48.81 |        9.4822 |                             1.7486 | 0.18s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     52.78 |        9.6599 |                             1.7204 | 0.16s  |
| IRS.FH (H=25)                                               |     54.09 |       10.0328 |                             2.1203 | 1.67s  |
| WR-SDA (forced_exploration=true)                            |     52.76 |       10.1746 |                             2.5789 | 0.86s  |
| WR-SDA (forced_exploration=false)                           |     52.23 |       10.2123 |                             2.7845 | 0.70s  |
| POKER (H=50)                                                |     41.34 |       10.6033 |                             2.7010 | 0.31s  |
| POKER (H=25)                                                |     41.02 |       10.6190 |                             2.7176 | 0.34s  |
| POKER (H=100)                                               |     41.56 |       10.6923 |                             2.7973 | 0.36s  |
| ϵ-Greedy (ϵ=0.050)                                          |     51.57 |       10.7107 |                             1.9386 | 0.12s  |
| POKER (H=250)                                               |     42.10 |       10.8128 |                             2.8381 | 0.32s  |
| ϵ-Exploring Thompson Sampling                               |     45.15 |       10.8883 |                             4.0500 | 0.17s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.5)                         |     49.54 |       10.9591 |                             1.8509 | 0.25s  |
| MOSS-Anytime (α=-0.85)                                      |     50.92 |       11.2098 |                             1.9147 | 0.18s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.1)                         |     48.79 |       11.4276 |                             1.8858 | 0.24s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     50.84 |       11.5088 |                             2.0720 | 0.15s  |
| KL-UCB                                                      |     51.49 |       11.6751 |                             3.5785 | 6.73s  |
| RS (a=0.99)                                                 |     52.43 |       12.1155 |                             6.1302 | 0.15s  |
| Softsatisficing (a=0.90)                                    |     28.86 |       12.3245 |                             7.4167 | 0.09s  |
| Thompson Sampling                                           |     48.51 |       12.4396 |                             2.7769 | 0.65s  |
| Weighted Bootstrap                                          |     48.57 |       12.4424 |                             2.7624 | 3.03s  |
| RAVEN-UCB (a0=0.5, b0=0.5, eps=0.001)                       |     47.05 |       12.5486 |                             1.8639 | 0.22s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     37.00 |       12.6026 |                             4.1618 | 15.16s |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     47.58 |       12.6638 |                             2.7907 | 0.89s  |
| ϵ-Greedy (ϵ=0.100)                                          |     49.34 |       12.7974 |                             2.3308 | 0.12s  |
| UCB-DT (γ=0.25)                                             |     48.56 |       12.8678 |                             2.0175 | 2.70s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.5)                          |     46.61 |       12.9612 |                             1.8873 | 0.25s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     45.49 |       13.1710 |                             2.8249 | 0.84s  |
| Batch Ensemble for MAB (m=2)                                |     41.46 |       13.3774 |                             9.4515 | 0.11s  |
| Non-Parametric Thompson Sampling                            |     47.46 |       13.6038 |                             4.3455 | 4.18s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.1)                          |     45.57 |       13.8469 |                             1.9559 | 0.25s  |
| BayesUCB (δ=0.300)                                          |     26.75 |       13.8625 |                             8.5920 | 0.19s  |
| Forced Exploration                                          |     48.30 |       13.9900 |                             2.5181 | 0.09s  |
| Least Failures                                              |     47.50 |       14.3230 |                             6.3554 | 0.07s  |
| Softsatisficing (a=0.99)                                    |     47.05 |       14.4903 |                             8.0153 | 0.26s  |
| Bounded Dirichlet Sampling                                  |     45.58 |       14.5418 |                             4.6561 | 2.00s  |
| FTPL-GR (lr=1.000)                                          |     44.10 |       14.7489 |                             2.7172 | 4.14s  |
| BayesUCB (δ=0.100)                                          |     37.27 |       15.0673 |                             2.3660 | 0.19s  |
| Kullback-Leibler Maillard Sampling                          |     43.53 |       15.1294 |                             5.1731 | 0.52s  |
| MOSS-Anytime (α=-0.50)                                      |     44.06 |       15.3933 |                             2.1697 | 0.23s  |
| Hellinger-UCB                                               |     43.81 |       15.5306 |                             5.4955 | 1.53s  |
| RAVEN-UCB (a0=0.5, b0=10, eps=0.001)                        |     44.04 |       15.6325 |                             2.3208 | 0.25s  |
| VarTS                                                       |     44.55 |       15.6709 |                             2.3704 | 0.46s  |
| MOSS-Anytime (α=-0.33)                                      |     40.99 |       17.0540 |                             2.2724 | 0.23s  |
| MARS (δ=1.000)                                              |     33.17 |       17.3184 |                             5.7422 | 0.09s  |
| Gittins Index -- Whittle's Approximation (β=0.99)           |     21.93 |       17.3442 |                            10.7921 | 0.23s  |
| UCBT                                                        |     32.33 |       18.1863 |                             6.0728 | 0.08s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.5)                             |     32.80 |       18.2035 |                             2.8245 | 0.26s  |
| Batch Ensemble for MAB (m=4)                                |     25.45 |       18.2890 |                            11.3150 | 0.13s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.1)                             |     32.55 |       18.5572 |                             2.8832 | 0.26s  |
| MARS (δ=0.100)                                              |     35.28 |       18.7945 |                             6.0606 | 0.32s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     23.97 |       18.9613 |                             5.2597 | 0.97s  |
| SoftElim (w=0.50)                                           |     29.57 |       19.3858 |                             3.0999 | 0.41s  |
| BayesUCB (δ=0.400)                                          |     19.76 |       19.4237 |                            12.4626 | 0.19s  |
| EB-TCI                                                      |     35.93 |       19.7395 |                             5.2415 | 0.28s  |
| UCB-DT (γ=0.10)                                             |     34.12 |       19.9626 |                             3.2406 | 2.63s  |
| ReBoot (r=0.25)                                             |     34.89 |       19.9697 |                             3.1894 | 0.25s  |
| RAVEN-UCB (a0=1, b0=5, eps=0.001)                           |     31.26 |       20.1089 |                             3.2838 | 0.26s  |
| Gittins Index -- Whittle's Approximation (β=0.90)           |     19.03 |       20.6196 |                            13.1752 | 0.26s  |
| Vanilla Residual Bootstrap (init=0)                         |     33.47 |       21.5207 |                             3.2212 | 0.18s  |
| Multiplier Bootstrap-based Exploration                      |     28.45 |       22.2710 |                             3.5416 | 6.20s  |
| ETC (m=20)                                                  |     33.55 |       22.3233 |                             4.2529 | 0.15s  |
| ETC (m=10)                                                  |     27.09 |       22.3539 |                             6.4168 | 0.15s  |
| ReBoot (r=0.50)                                             |     30.87 |       22.5161 |                             3.8147 | 0.23s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     38.74 |       22.6530 |                             4.3599 | 0.13s  |
| UCB1-Tuned                                                  |     25.07 |       22.9077 |                             3.4824 | 0.19s  |
| Vanilla Residual Bootstrap (init=1)                         |     31.19 |       23.1578 |                             3.3412 | 0.21s  |
| Tsallis-INF                                                 |     26.30 |       23.2635 |                             4.3108 | 1.05s  |
| MARS (δ=0.010)                                              |     26.79 |       23.5506 |                             7.7650 | 1.78s  |
| Vanilla Residual Bootstrap (init=5)                         |     30.48 |       23.8825 |                             3.5154 | 0.27s  |
| Garbage In, Reward Out (a=0.10)                             |     26.82 |       23.9510 |                             3.8778 | 0.83s  |
| MARS (δ=0.001)                                              |     23.58 |       24.5993 |                             9.9033 | 10.68s |
| Batch Ensemble for MAB (m=8)                                |     19.96 |       24.6930 |                            17.4961 | 0.16s  |
| Perturbed-History Exploration (a=1.1)                       |     24.17 |       24.8624 |                             4.3134 | 0.93s  |
| MARS (δ=0.002)                                              |     23.66 |       25.1527 |                             9.2101 | 5.55s  |
| RS (a=0.75)                                                 |     16.69 |       25.2082 |                            15.9762 | 0.18s  |
| BayesUCB (δ=0.500)                                          |     15.86 |       25.3034 |                            16.2815 | 0.20s  |
| FTPL-GR (lr=0.100)                                          |     26.18 |       25.4951 |                             4.2832 | 4.25s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     12.50 |       26.5896 |                             8.8139 | 0.95s  |
| ETC (m=25)                                                  |     28.64 |       27.0247 |                             5.2417 | 0.15s  |
| CODE (δ=0.900)                                              |     16.26 |       27.7259 |                             4.4425 | 0.20s  |
| Garbage In, Reward Out (a=0.33)                             |     21.22 |       28.0093 |                             4.7583 | 1.14s  |
| Bootstrapped Thompson Sampling (J=500)                      |     17.05 |       28.0954 |                            16.5475 | 4.26s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     16.11 |       28.1867 |                            16.6249 | 8.29s  |
| ReBoot (r=0.90)                                             |     24.08 |       28.2376 |                             5.0547 | 0.28s  |
| lil' UCB (δ=0.100)                                          |     19.19 |       28.5694 |                             4.7509 | 0.28s  |
| Bootstrapped Thompson Sampling (J=100)                      |     15.82 |       29.0489 |                            16.7117 | 1.04s  |
| ReBoot (r=1.00)                                             |     22.53 |       29.7884 |                             5.3791 | 0.23s  |
| Softsatisficing (a=0.75)                                    |     14.24 |       30.0109 |                            19.0522 | 0.07s  |
| Bootstrapped Thompson Sampling (J=10)                       |     15.12 |       30.0861 |                            17.7177 | 0.43s  |
| Gittins Index -- Whittle's Approximation (β=0.70)           |     14.69 |       30.0926 |                            19.2525 | 0.25s  |
| Perturbed-History Exploration (a=2.1)                       |     18.72 |       30.3983 |                             5.2058 | 1.11s  |
| SoftElim (w=1.00)                                           |     16.81 |       30.6976 |                             5.1339 | 0.41s  |
| lil' UCB (δ=0.010)                                          |     16.72 |       32.2288 |                             5.5208 | 0.28s  |
| Garbage In, Reward Out (a=1.00)                             |     17.26 |       32.4632 |                             5.6672 | 1.20s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     27.51 |       32.6383 |                             6.3517 | 0.08s  |
| Boltzmann-Gumbel Exploration                                |     17.44 |       32.7460 |                             5.6438 | 0.33s  |
| Gradient Bandit                                             |     18.28 |       33.0070 |                             6.4322 | 0.36s  |
| Gittins Index -- Whittle's Approximation (β=0.50)           |     13.79 |       34.0919 |                            21.5379 | 0.27s  |
| lil' UCB (δ=0.001)                                          |     15.51 |       34.2797 |                             5.8524 | 0.26s  |
| Gradient Bandit (with baseline)                             |     17.93 |       34.4364 |                             5.7630 | 0.44s  |
| EXP-IX                                                      |     15.62 |       34.8327 |                             6.2311 | 0.44s  |
| ReBoot (r=1.50)                                             |     18.20 |       35.2644 |                             6.5573 | 0.25s  |
| Perturbed-History Exploration (a=5.1)                       |     15.29 |       35.4831 |                             6.2519 | 1.01s  |
| UCB1                                                        |     14.55 |       36.1248 |                             6.3580 | 0.16s  |
| ReBoot (r=1.70)                                             |     17.25 |       36.7828 |                             6.9301 | 0.33s  |
| RS (a=0.65)                                                 |     12.54 |       36.9123 |                            23.2223 | 0.17s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.95)    |     12.48 |       37.2813 |                            23.4074 | 0.48s  |
| Softsatisficing (a=0.65)                                    |     12.05 |       38.0281 |                            23.7792 | 0.07s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.9)     |     12.42 |       38.3118 |                            23.9070 | 0.44s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.5)                             |     13.50 |       38.3221 |                             6.8900 | 0.27s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.1)                             |     13.50 |       38.3307 |                             6.8949 | 0.28s  |
| RAVEN-UCB (a0=5, b0=1, eps=0.001)                           |     13.47 |       38.4149 |                             6.9164 | 0.27s  |
| Gittins Index -- Brezzi and Lai's Approximation (β=0.8)     |     12.38 |       38.6260 |                            24.0049 | 0.46s  |
| BayesUCB (δ=0.900)                                          |     12.29 |       38.7864 |                            24.1887 | 0.19s  |
| ReBoot (r=2.10)                                             |     15.90 |       39.2124 |                             7.5247 | 0.31s  |
| SoftElim (w=2.00)                                           |     12.25 |       39.8744 |                             7.1264 | 0.41s  |
| UCB-DT (γ=0.02)                                             |     13.53 |       39.8999 |                             7.4970 | 2.39s  |
| ETC (m=5)                                                   |     12.36 |       41.7571 |                             9.1900 | 0.18s  |
| RS (a=0.50)                                                 |     11.80 |       42.0199 |                            25.5228 | 0.17s  |
| FTPL-GR (lr=0.010)                                          |     12.28 |       42.4732 |                             8.0136 | 4.42s  |
| SoftElim (w=2.83)                                           |     11.27 |       43.0977 |                             7.9490 | 0.40s  |
| ETC (m=3)                                                   |     12.03 |       43.5920 |                             9.6906 | 0.24s  |
| Softsatisficing (a=0.50)                                    |     11.18 |       43.6892 |                            26.5617 | 0.08s  |
| ETC (m=2)                                                   |     11.03 |       45.2564 |                             9.3287 | 0.12s  |
| RS (a=0.25)                                                 |     11.31 |       46.4787 |                            27.3045 | 0.18s  |
| Softsatisficing (a=0.25)                                    |     11.03 |       46.8837 |                            27.6280 | 0.07s  |
| Softsatisficing (a=0.10)                                    |     10.99 |       47.1460 |                            27.6826 | 0.05s  |
| RS (a=0.10)                                                 |     11.22 |       47.2198 |                            27.6594 | 0.16s  |
| RS (a=0.00)                                                 |     11.13 |       47.2636 |                            27.6526 | 0.13s  |
| FTPL-GR (lr=0.001)                                          |     10.23 |       48.4041 |                             9.6683 | 4.30s  |
| CODE (δ=0.050)                                              |     10.00 |       49.2639 |                             9.8811 | 0.18s  |
| Random                                                      |      9.99 |       49.2870 |                            10.0029 | 0.02s  |

<!-- END mdsh -->

</details>

## Notes / Conclusions

- I have probably made mistakes while translating the formulas from the papers
  into code – KL-UCB, PHE, GIRO, etc. are doing much worse than I expected.

- Keep in mind that the experiments were on the Bernoulli Bandit with a short
  horizon only. Many of the algorithms also work on other kinds of bandits and
  may rank differently there.

- Greedy algorithms seem to do well on a short horizon in general, but fall
  behind after a few hundred steps.

- UCB-DT does very well; the tuning parameter can be adjusted for the length
  of the horizon. More greedy settings (as used above) seem to do well over
  short time horizons.

- ϵ-Exploring Thompson Sampling seems to match or exceed Thompson Sampling
  while being computationally much lighter.

- Augmenting TS-UCB with the ϵ-Exploring policy speeds up TS-UCB by a lot, but
  trades off a bit of performance. For reasons I'm not entirely sure of, fewer
  samples seem to perform just as well or better than the variants with more samples.
