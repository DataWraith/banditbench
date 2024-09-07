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

### Bootstrap-based

- [Bootstrapped Thompson Sampling](https://arxiv.org/abs/1410.4009)
- [Garbage In, Reward Out](http://proceedings.mlr.press/v97/kveton19a/kveton19a.pdf) (PDF)
- [Multiplier Bootstrap-based Exploration](https://arxiv.org/abs/2302.01543)
- [Perturbed-History Exploration](https://arxiv.org/abs/1902.10089)
- [ReBoot](https://arxiv.org/abs/2002.08436)
- Vanilla Residual Bootstrap

### Dueling-based

- [Bounded Dirichlet Sampling](https://arxiv.org/abs/2111.09724)
- [WR-SDA](https://arxiv.org/abs/2010.14323)

### Thompson Sampling-based

- [ϵ-Exploring Thompson Sampling](https://proceedings.mlr.press/v202/jin23b/jin23b.pdf) (PDF)
- [Information Relaxation Sampling (Finite Horizon)](https://arxiv.org/abs/1902.04251) (simplified version that uses an assumed, fixed horizon)
- [Non-Parametric Thompson Sampling](https://proceedings.mlr.press/v117/riou20a.html)
- Optimistic Thompson Sampling
- [Satisficing Thompson Sampling](https://arxiv.org/abs/1704.09028)
- [Thompson Sampling with Virtual Helping Agents (C3)](https://arxiv.org/abs/2209.08197)
- Thompson Sampling

### Upper Confidence Bound-based

- [BayesUCB](https://arxiv.org/abs/2306.09136)
- [Hellinger-UCB](https://arxiv.org/abs/2404.10207)
- [KL-UCB](https://arxiv.org/abs/1102.2490)
- [lil' UCB](https://arxiv.org/abs/1312.7308)
- [MOSS-anytime](http://proceedings.mlr.press/v48/degenne16.html)
- [UCB1](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [UCB1-Tuned](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [UCB-DT](https://arxiv.org/abs/2110.02690)
- [UCBT](https://arxiv.org/abs/2102.05263)

### Other

- [Boltzmann-Gumbel Exploration](https://arxiv.org/abs/1705.10257)
- [CODE](https://arxiv.org/abs/2310.14751)
- [EB-TCI](https://arxiv.org/abs/2206.05979)
- [EXP-IX](https://arxiv.org/abs/1506.03271)
- [Forced Exploration](https://arxiv.org/abs/2312.07285)
- [Gradient Bandit](https://arxiv.org/abs/2402.17235)
- [Kullback-Leibler Maillard Sampling](https://arxiv.org/abs/2304.14989)
- [Parameter Free Learning Automaton](https://arxiv.org/abs/1711.10111)
- [POKER](https://link.springer.com/chapter/10.1007/11564096_42) (with fixed Horizon)
- [Tsallis-INF](https://arxiv.org/abs/1807.07623)
- [TS-UCB](https://arxiv.org/abs/2006.06372)

## Results

The following table shows the average rank and runtime of each algorithm when
considering the five experiments further down in this file.

<!-- `> cat aggregated_ranks.md` -->

<!-- BEGIN mdsh -->
| Algorithm                                                   | Average Rank | Average Time (seconds) |
| ----------------------------------------------------------- | ------------ | ---------------------- |
| IRS.FH (H=2)                                                | 10.2         | 1.37                   |
| IRS.FH (H=3)                                                | 12.0         | 1.48                   |
| IRS.FH (H=1)                                                | 12.2         | 1.29                   |
| UCB-DT (γ=0.90)                                             | 12.2         | 4.62                   |
| UCB-DT (γ=0.95)                                             | 12.8         | 4.65                   |
| TS-UCB (100 samples)                                        | 13.0         | 82.33                  |
| UCB-DT (γ=0.75)                                             | 13.8         | 4.67                   |
| UCB-DT (γ=1.00)                                             | 14.8         | 4.3                    |
| ϵ-Exploring TS-UCB (1 samples)                              | 16.0         | 0.2                    |
| IRS.FH (H=4)                                                | 16.4         | 1.39                   |
| ϵ-Exploring TS-UCB (10 samples)                             | 16.4         | 0.97                   |
| ϵ-Exploring TS-UCB (100 samples)                            | 16.6         | 8.33                   |
| TS-UCB (10 samples)                                         | 19.6         | 8.36                   |
| MOSS-Anytime (α=-0.85)                                      | 20.4         | 0.24                   |
| BayesUCB (δ=0.300)                                          | 23.0         | 0.24                   |
| IRS.FH (H=5)                                                | 23.0         | 1.51                   |
| BayesUCB (δ=0.200)                                          | 23.8         | 0.25                   |
| BayesUCB (δ=0.400)                                          | 24.2         | 0.25                   |
| ϵ-Decreasing (ϵ=0.990)                                      | 25.6         | 0.18                   |
| Greedy                                                      | 25.8         | 0.09                   |
| POKER (H=10)                                                | 26.0         | 0.34                   |
| POKER (H=5)                                                 | 26.2         | 0.34                   |
| CODE (δ=0.990)                                              | 26.6         | 0.4                    |
| POKER (H=1)                                                 | 26.6         | 0.31                   |
| ϵ-Decreasing (ϵ=0.900)                                      | 27.0         | 0.18                   |
| ϵ-Decreasing (ϵ=0.700)                                      | 27.4         | 0.18                   |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) | 27.6         | 18.09                  |
| MOSS-Anytime (α=-0.50)                                      | 28.4         | 0.28                   |
| BayesUCB (δ=0.500)                                          | 29.8         | 0.24                   |
| ϵ-Greedy (ϵ=0.010)                                          | 29.8         | 0.09                   |
| TS-UCB (1 samples)                                          | 30.6         | 1.03                   |
| ϵ-Greedy (ϵ=0.020)                                          | 30.6         | 0.13                   |
| ϵ-Greedy (ϵ=0.050)                                          | 32.6         | 0.13                   |
| IRS.FH (H=10)                                               | 33.6         | 1.51                   |
| POKER (H=25)                                                | 34.2         | 0.34                   |
| ϵ-Decreasing (ϵ=0.500)                                      | 34.4         | 0.28                   |
| MOSS-Anytime (α=-0.33)                                      | 34.8         | 0.28                   |
| ϵ-Exploring Thompson Sampling                               | 35.2         | 0.17                   |
| POKER (H=50)                                                | 37.2         | 0.35                   |
| POKER (H=100)                                               | 37.6         | 0.34                   |
| ϵ-Greedy (ϵ=0.100)                                          | 40.8         | 0.12                   |
| UCBT                                                        | 41.2         | 0.11                   |
| BayesUCB (δ=0.100)                                          | 43.0         | 0.21                   |
| IRS.FH (H=25)                                               | 43.4         | 1.83                   |
| BayesUCB (δ=0.900)                                          | 44.2         | 0.24                   |
| Forced Exploration                                          | 44.8         | 0.09                   |
| Optimistic Thompson Sampling                                | 45.4         | 1.07                   |
| WR-SDA                                                      | 45.6         | 1.48                   |
| ReBoot (r=0.25)                                             | 46.0         | 0.26                   |
| POKER (H=250)                                               | 49.6         | 0.35                   |
| ReBoot (r=0.50)                                             | 51.8         | 0.3                    |
| Bootstrapped Thompson Sampling (J=10)                       | 52.4         | 0.39                   |
| Thompson Sampling                                           | 52.6         | 0.82                   |
| Satisficing Thompson Sampling (ϵ=0.005)                     | 53.4         | 1.04                   |
| Satisficing Thompson Sampling (ϵ=0.010)                     | 54.4         | 1.08                   |
| Hellinger-UCB                                               | 54.4         | 2.34                   |
| Vanilla Residual Bootstrap (init=0)                         | 54.6         | 0.22                   |
| Bootstrapped Thompson Sampling (J=500)                      | 54.6         | 4.49                   |
| Bootstrapped Thompson Sampling (J=1000)                     | 55.8         | 8.91                   |
| Bootstrapped Thompson Sampling (J=100)                      | 56.0         | 1.16                   |
| KL-UCB                                                      | 56.4         | 9.43                   |
| Multiplier Bootstrap-based Exploration                      | 56.4         | 6.02                   |
| Non-Parametric Thompson Sampling                            | 58.0         | 4.68                   |
| CODE (δ=0.900)                                              | 58.4         | 0.44                   |
| UCB1-Tuned                                                  | 59.2         | 0.34                   |
| Garbage In, Reward Out (a=0.10)                             | 60.4         | 1.09                   |
| Satisficing Thompson Sampling (ϵ=0.050)                     | 63.2         | 1.14                   |
| Bounded Dirichlet Sampling                                  | 63.6         | 2.59                   |
| Vanilla Residual Bootstrap (init=1)                         | 63.8         | 0.3                    |
| ϵ-Decreasing (ϵ=0.200)                                      | 65.8         | 0.16                   |
| Kullback-Leibler Maillard Sampling                          | 66.2         | 0.64                   |
| Perturbed-History Exploration (a=1.1)                       | 68.0         | 0.98                   |
| EB-TCI                                                      | 68.4         | 0.36                   |
| ETC (m=10)                                                  | 73.4         | 0.15                   |
| ReBoot (r=0.90)                                             | 74.0         | 0.3                    |
| Tsallis-INF                                                 | 74.4         | 2.38                   |
| Garbage In, Reward Out (a=0.33)                             | 75.4         | 1.24                   |
| Satisficing Thompson Sampling (ϵ=0.100)                     | 76.6         | 1.14                   |
| lil' UCB (δ=0.100)                                          | 77.2         | 0.38                   |
| Vanilla Residual Bootstrap (init=5)                         | 79.0         | 0.27                   |
| ReBoot (r=1.00)                                             | 79.4         | 0.3                    |
| PFLA (n=100)                                                | 80.6         | 89.87                  |
| Perturbed-History Exploration (a=2.1)                       | 81.8         | 1.09                   |
| ETC (m=20)                                                  | 82.6         | 0.14                   |
| ETC (m=5)                                                   | 83.2         | 0.15                   |
| ϵ-Decreasing (ϵ=0.100)                                      | 83.4         | 0.12                   |
| lil' UCB (δ=0.010)                                          | 85.8         | 0.39                   |
| Garbage In, Reward Out (a=1.00)                             | 86.4         | 1.27                   |
| ETC (m=25)                                                  | 86.4         | 0.15                   |
| PFLA (n=10)                                                 | 87.0         | 9.33                   |
| Boltzmann-Gumbel Exploration                                | 87.2         | 0.43                   |
| ReBoot (r=1.50)                                             | 88.0         | 0.3                    |
| lil' UCB (δ=0.001)                                          | 90.0         | 0.38                   |
| ReBoot (r=1.70)                                             | 90.6         | 0.29                   |
| UCB1                                                        | 94.0         | 0.21                   |
| ETC (m=3)                                                   | 94.0         | 0.14                   |
| ReBoot (r=2.10)                                             | 94.2         | 0.31                   |
| EXP-IX                                                      | 95.0         | 0.6                    |
| ETC (m=2)                                                   | 96.2         | 0.11                   |
| Gradient Bandit                                             | 97.4         | 0.51                   |
| Gradient Bandit (with baseline)                             | 98.6         | 0.53                   |
| PFLA (n=1)                                                  | 102.2        | 1.15                   |
| Random                                                      | 103.0        | 0.02                   |
| CODE (δ=0.050)                                              | 103.2        | 0.42                   |
<!-- END mdsh -->

## Data

### Uniform

This experiment uses 10 arms, with the means sampled uniformly from the interval
\[0, 1\]. This is a relatively easy instance, because there is likely to be a
single best arm that is easy to find. This is reflected in the %-Optimal column,
where the best algorithms reach over 2/3 pull rate of the optimal arm.

<details>
<summary>Results</summary>

<!-- `> cat uniform.md` -->

<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| BayesUCB (δ=0.300)                                          |     68.64 |       16.4619 |                             5.4296 | 0.23s  |
| TS-UCB (100 samples)                                        |     72.02 |       16.8866 |                             3.3722 | 82.63s |
| IRS.FH (H=2)                                                |     71.14 |       16.9375 |                             2.7279 | 1.47s  |
| IRS.FH (H=3)                                                |     72.29 |       16.9395 |                             3.0432 | 1.42s  |
| BayesUCB (δ=0.400)                                          |     64.26 |       17.1313 |                             6.9164 | 0.24s  |
| TS-UCB (10 samples)                                         |     72.48 |       17.2679 |                             3.7317 | 8.29s  |
| BayesUCB (δ=0.200)                                          |     71.21 |       17.5846 |                             4.0029 | 0.24s  |
| IRS.FH (H=4)                                                |     72.57 |       17.6097 |                             3.5134 | 1.48s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     69.94 |       17.8335 |                             2.8808 | 8.79s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     70.16 |       17.9184 |                             2.9572 | 1.04s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     70.36 |       18.0420 |                             3.0933 | 0.21s  |
| IRS.FH (H=1)                                                |     68.89 |       18.0936 |                             2.5509 | 1.37s  |
| UCB-DT (γ=1.00)                                             |     69.93 |       18.1466 |                             2.5287 | 4.27s  |
| UCB-DT (γ=0.95)                                             |     72.44 |       18.1946 |                             2.4725 | 3.98s  |
| UCB-DT (γ=0.75)                                             |     72.50 |       18.1962 |                             2.5172 | 3.83s  |
| UCB-DT (γ=0.90)                                             |     72.42 |       18.2016 |                             2.4807 | 3.87s  |
| IRS.FH (H=5)                                                |     72.59 |       18.3624 |                             4.0320 | 1.70s  |
| MOSS-Anytime (α=-0.85)                                      |     69.71 |       18.8113 |                             2.5659 | 0.25s  |
| CODE (δ=0.990)                                              |     68.91 |       18.9329 |                             2.9569 | 0.38s  |
| POKER (H=10)                                                |     65.48 |       19.3526 |                             3.2000 | 0.42s  |
| POKER (H=5)                                                 |     66.34 |       19.3558 |                             2.7035 | 0.38s  |
| TS-UCB (1 samples)                                          |     71.83 |       19.5545 |                             5.3564 | 1.07s  |
| POKER (H=1)                                                 |     66.55 |       19.5748 |                             2.5553 | 0.35s  |
| Greedy                                                      |     66.26 |       19.7129 |                             2.5470 | 0.09s  |
| BayesUCB (δ=0.500)                                          |     58.32 |       20.1518 |                             9.3444 | 0.23s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     66.35 |       20.7765 |                             2.7735 | 0.19s  |
| BayesUCB (δ=0.100)                                          |     68.95 |       20.9900 |                             3.7246 | 0.20s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     63.16 |       21.1040 |                             6.1932 | 31.60s |
| ϵ-Greedy (ϵ=0.010)                                          |     66.18 |       21.1769 |                             2.8588 | 0.09s  |
| IRS.FH (H=10)                                               |     71.83 |       21.2519 |                             5.4688 | 1.59s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     66.48 |       21.2824 |                             2.8492 | 0.18s  |
| POKER (H=25)                                                |     61.38 |       21.3386 |                             6.6186 | 0.40s  |
| MOSS-Anytime (α=-0.50)                                      |     70.74 |       22.4582 |                             2.7088 | 0.27s  |
| ϵ-Greedy (ϵ=0.020)                                          |     65.99 |       22.7752 |                             3.1672 | 0.12s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     66.55 |       23.6847 |                             3.3687 | 0.19s  |
| WR-SDA                                                      |     66.87 |       23.8280 |                             5.0922 | 1.26s  |
| IRS.FH (H=25)                                               |     70.09 |       24.4452 |                             6.0015 | 1.84s  |
| MOSS-Anytime (α=-0.33)                                      |     69.75 |       24.4536 |                             2.6909 | 0.27s  |
| Optimistic Thompson Sampling                                |     68.80 |       25.6235 |                             7.1784 | 1.12s  |
| POKER (H=50)                                                |     56.21 |       25.6788 |                             9.6913 | 0.39s  |
| ϵ-Greedy (ϵ=0.050)                                          |     65.45 |       27.3929 |                             4.0210 | 0.12s  |
| ϵ-Exploring Thompson Sampling                               |     62.82 |       27.9018 |                             9.2377 | 0.19s  |
| UCBT                                                        |     65.40 |       28.7984 |                             4.0759 | 0.11s  |
| Thompson Sampling                                           |     66.16 |       28.8956 |                             7.1444 | 0.84s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     65.94 |       29.0318 |                             7.1008 | 1.09s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     65.61 |       29.3229 |                             7.0179 | 1.12s  |
| KL-UCB                                                      |     66.78 |       29.6304 |                             7.3837 | 10.11s |
| ReBoot (r=0.25)                                             |     61.18 |       30.3599 |                             5.2731 | 0.29s  |
| CODE (δ=0.900)                                              |     54.94 |       30.6423 |                             6.5536 | 0.42s  |
| POKER (H=100)                                               |     51.57 |       30.8991 |                            12.6895 | 0.36s  |
| Hellinger-UCB                                               |     63.89 |       31.0005 |                             7.0702 | 2.66s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     65.55 |       31.3306 |                             4.6232 | 0.29s  |
| UCB1-Tuned                                                  |     62.03 |       31.6747 |                             3.6906 | 0.31s  |
| Vanilla Residual Bootstrap (init=0)                         |     59.99 |       33.1442 |                             5.4073 | 0.21s  |
| Non-Parametric Thompson Sampling                            |     63.70 |       33.7962 |                             7.1820 | 4.92s  |
| ReBoot (r=0.50)                                             |     58.58 |       34.0829 |                             5.9224 | 0.32s  |
| Bounded Dirichlet Sampling                                  |     63.86 |       34.1647 |                             7.1345 | 2.55s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     57.19 |       35.0506 |                             6.7983 | 1.24s  |
| ϵ-Greedy (ϵ=0.100)                                          |     63.98 |       35.8380 |                             5.3322 | 0.12s  |
| Multiplier Bootstrap-based Exploration                      |     60.70 |       36.1612 |                             4.2418 | 6.13s  |
| Kullback-Leibler Maillard Sampling                          |     59.67 |       37.5162 |                             8.3979 | 0.65s  |
| Perturbed-History Exploration (a=1.1)                       |     56.96 |       37.8929 |                             5.6711 | 0.86s  |
| POKER (H=250)                                               |     46.27 |       38.6838 |                            15.5508 | 0.37s  |
| Garbage In, Reward Out (a=0.10)                             |     57.65 |       38.7302 |                             5.2772 | 1.05s  |
| BayesUCB (δ=0.900)                                          |     39.28 |       40.0985 |                            22.3598 | 0.24s  |
| Vanilla Residual Bootstrap (init=1)                         |     59.43 |       40.6304 |                             4.7837 | 0.30s  |
| Bootstrapped Thompson Sampling (J=500)                      |     40.59 |       41.9370 |                            21.7066 | 4.77s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     40.88 |       41.9668 |                            21.1936 | 9.73s  |
| Bootstrapped Thompson Sampling (J=100)                      |     40.77 |       42.3584 |                            21.7453 | 1.43s  |
| Bootstrapped Thompson Sampling (J=10)                       |     39.55 |       42.8224 |                            21.8677 | 0.47s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     44.13 |       44.2992 |                            10.4673 | 1.29s  |
| lil' UCB (δ=0.100)                                          |     52.19 |       44.8365 |                             5.5606 | 0.40s  |
| Tsallis-INF                                                 |     54.25 |       46.4787 |                             5.9697 | 2.17s  |
| Forced Exploration                                          |     62.89 |       46.6666 |                             6.2607 | 0.09s  |
| ReBoot (r=0.90)                                             |     52.24 |       47.2795 |                             6.7367 | 0.30s  |
| Garbage In, Reward Out (a=0.33)                             |     51.74 |       49.2706 |                             5.5459 | 1.38s  |
| Vanilla Residual Bootstrap (init=5)                         |     55.69 |       50.7442 |                             6.1208 | 0.26s  |
| ReBoot (r=1.00)                                             |     49.90 |       51.8800 |                             6.7533 | 0.32s  |
| EB-TCI                                                      |     42.82 |       55.0174 |                            15.7714 | 0.40s  |
| Perturbed-History Exploration (a=2.1)                       |     47.44 |       56.5448 |                             6.0521 | 1.04s  |
| ETC (m=10)                                                  |     47.32 |       56.6956 |                            11.0554 | 0.16s  |
| lil' UCB (δ=0.010)                                          |     44.08 |       62.1486 |                             6.5312 | 0.40s  |
| Garbage In, Reward Out (a=1.00)                             |     43.03 |       66.4802 |                             6.9482 | 1.32s  |
| Boltzmann-Gumbel Exploration                                |     43.87 |       68.9250 |                             6.5817 | 0.46s  |
| ReBoot (r=1.50)                                             |     40.44 |       72.1794 |                             8.1305 | 0.31s  |
| lil' UCB (δ=0.001)                                          |     39.18 |       73.8291 |                             8.0325 | 0.40s  |
| ETC (m=5)                                                   |     27.93 |       78.7963 |                            24.1796 | 0.16s  |
| ReBoot (r=1.70)                                             |     37.41 |       79.4522 |                             8.9230 | 0.31s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     50.82 |       81.7548 |                            11.1762 | 0.16s  |
| ETC (m=20)                                                  |     49.52 |       85.1694 |                            11.9964 | 0.17s  |
| UCB1                                                        |     34.52 |       86.8474 |                            10.2054 | 0.19s  |
| PFLA (n=100)                                                |     29.61 |       91.1676 |                            38.5302 | 96.79s |
| ReBoot (r=2.10)                                             |     32.31 |       92.8131 |                            10.7156 | 0.32s  |
| EXP-IX                                                      |     31.87 |       95.7830 |                            13.0250 | 0.62s  |
| ETC (m=3)                                                   |     22.30 |       98.5252 |                            27.0722 | 0.15s  |
| ETC (m=25)                                                  |     41.95 |      105.2629 |                            14.8396 | 0.17s  |
| ETC (m=2)                                                   |     20.21 |      110.5641 |                            26.8868 | 0.11s  |
| Gradient Bandit                                             |     30.32 |      110.7043 |                            17.1641 | 0.55s  |
| Gradient Bandit (with baseline)                             |     31.34 |      113.5984 |                            11.7063 | 0.61s  |
| PFLA (n=10)                                                 |     23.62 |      121.0144 |                            55.8349 | 10.68s |
| ϵ-Decreasing (ϵ=0.100)                                      |     35.59 |      127.2145 |                            17.7947 | 0.12s  |
| CODE (δ=0.050)                                              |     10.94 |      187.9726 |                            24.8420 | 0.41s  |
| PFLA (n=1)                                                  |     10.56 |      200.9131 |                            29.7225 | 1.23s  |
| Random                                                      |     10.01 |      204.0160 |                            30.3495 | 0.02s  |
<!-- END mdsh -->

</details>

### Half-Range

This experiment uses 10 arms, with the means sampled uniformly from the interval
\[0.25, 0.75\]. This is a harder instance, because the arms are closer together
and thus harder to distinguish.

This experiment was taken from the GIRO paper.

<details>
<summary>Results</summary>

<!-- `> cat half_range.md` -->

<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| IRS.FH (H=2)                                                |     43.85 |       24.7422 |                             7.8222 | 1.33s  |
| IRS.FH (H=3)                                                |     45.28 |       25.0047 |                             6.6326 | 1.54s  |
| UCB-DT (γ=0.90)                                             |     43.02 |       25.6120 |                             7.2004 | 4.97s  |
| UCB-DT (γ=0.95)                                             |     43.00 |       25.6319 |                             7.1816 | 4.98s  |
| UCB-DT (γ=0.75)                                             |     43.05 |       25.6700 |                             7.2075 | 5.01s  |
| BayesUCB (δ=0.900)                                          |     39.16 |       25.8336 |                            13.1050 | 0.24s  |
| BayesUCB (δ=0.500)                                          |     42.67 |       26.1689 |                             8.4137 | 0.24s  |
| BayesUCB (δ=0.400)                                          |     43.04 |       26.4233 |                             8.1311 | 0.24s  |
| BayesUCB (δ=0.300)                                          |     44.61 |       26.4883 |                             7.0859 | 0.24s  |
| IRS.FH (H=4)                                                |     44.59 |       26.4971 |                             6.7171 | 1.37s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     44.11 |       26.7246 |                             8.7506 | 13.07s |
| TS-UCB (100 samples)                                        |     45.02 |       26.9097 |                             6.2137 | 78.44s |
| IRS.FH (H=1)                                                |     39.73 |       26.9134 |                             9.8115 | 1.24s  |
| BayesUCB (δ=0.200)                                          |     45.32 |       27.2126 |                             6.1882 | 0.24s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     42.08 |       27.2128 |                             8.1379 | 0.19s  |
| MOSS-Anytime (α=-0.85)                                      |     40.04 |       27.3181 |                             8.7262 | 0.23s  |
| MOSS-Anytime (α=-0.50)                                      |     44.05 |       27.4891 |                             5.4358 | 0.26s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     41.55 |       27.5450 |                             8.1473 | 0.93s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     41.08 |       27.6722 |                             8.2739 | 7.94s  |
| CODE (δ=0.990)                                              |     39.41 |       27.7728 |                            10.1499 | 0.40s  |
| TS-UCB (10 samples)                                         |     44.55 |       27.9363 |                             5.9386 | 7.99s  |
| UCB-DT (γ=1.00)                                             |     38.52 |       28.0522 |                             9.8213 | 4.14s  |
| IRS.FH (H=5)                                                |     43.60 |       28.0814 |                             6.8634 | 1.38s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     38.24 |       28.1487 |                             9.6145 | 0.18s  |
| Greedy                                                      |     37.83 |       28.2076 |                             9.9996 | 0.08s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     38.32 |       28.3069 |                             9.4761 | 0.18s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     39.35 |       28.3077 |                             8.7988 | 0.17s  |
| POKER (H=1)                                                 |     37.76 |       28.3667 |                            10.1082 | 0.31s  |
| POKER (H=5)                                                 |     37.76 |       28.3800 |                            10.0953 | 0.33s  |
| POKER (H=10)                                                |     37.74 |       28.4050 |                            10.0473 | 0.33s  |
| ϵ-Greedy (ϵ=0.010)                                          |     38.03 |       28.4793 |                             9.7905 | 0.09s  |
| ϵ-Greedy (ϵ=0.020)                                          |     38.36 |       28.6900 |                             9.4808 | 0.12s  |
| POKER (H=25)                                                |     37.49 |       28.8412 |                             9.4550 | 0.35s  |
| ϵ-Greedy (ϵ=0.050)                                          |     39.46 |       29.3486 |                             8.7084 | 0.12s  |
| Bootstrapped Thompson Sampling (J=10)                       |     38.57 |       29.4073 |                            13.9756 | 0.40s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     40.91 |       29.4333 |                             7.5048 | 0.26s  |
| MOSS-Anytime (α=-0.33)                                      |     42.29 |       29.8866 |                             5.9957 | 0.27s  |
| POKER (H=100)                                               |     38.92 |       29.9131 |                             6.5647 | 0.38s  |
| BayesUCB (δ=0.100)                                          |     42.74 |       30.3237 |                             6.0061 | 0.21s  |
| POKER (H=50)                                                |     36.98 |       30.4262 |                             8.3416 | 0.36s  |
| ϵ-Exploring Thompson Sampling                               |     40.14 |       30.7657 |                             8.9988 | 0.16s  |
| Bootstrapped Thompson Sampling (J=500)                      |     38.36 |       30.8943 |                            13.6813 | 4.47s  |
| Bootstrapped Thompson Sampling (J=100)                      |     38.23 |       30.9704 |                            13.6387 | 1.05s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     37.93 |       31.2238 |                            13.7505 | 9.26s  |
| ϵ-Greedy (ϵ=0.100)                                          |     40.16 |       31.5381 |                             7.6639 | 0.12s  |
| TS-UCB (1 samples)                                          |     41.21 |       31.8313 |                             6.2230 | 0.99s  |
| IRS.FH (H=10)                                               |     41.13 |       32.0134 |                             6.6906 | 1.56s  |
| UCBT                                                        |     41.92 |       32.0754 |                             5.3843 | 0.10s  |
| Forced Exploration                                          |     41.72 |       33.1699 |                             5.7046 | 0.08s  |
| POKER (H=250)                                               |     37.22 |       33.9079 |                             8.0820 | 0.38s  |
| WR-SDA                                                      |     37.74 |       34.3702 |                             7.8470 | 1.84s  |
| IRS.FH (H=25)                                               |     38.87 |       35.4924 |                             6.7519 | 2.03s  |
| CODE (δ=0.900)                                              |     35.87 |       35.7202 |                            11.4984 | 0.42s  |
| UCB1-Tuned                                                  |     38.36 |       36.0304 |                             5.8517 | 0.34s  |
| ReBoot (r=0.25)                                             |     35.81 |       36.8892 |                             8.1828 | 0.23s  |
| Vanilla Residual Bootstrap (init=0)                         |     35.10 |       38.0391 |                             7.9288 | 0.21s  |
| Optimistic Thompson Sampling                                |     36.78 |       38.4207 |                             7.1289 | 1.02s  |
| Multiplier Bootstrap-based Exploration                      |     36.05 |       38.7066 |                             7.0003 | 6.42s  |
| ReBoot (r=0.50)                                             |     34.21 |       39.5480 |                             8.2009 | 0.30s  |
| ETC (m=10)                                                  |     33.45 |       40.0881 |                            11.7950 | 0.15s  |
| Hellinger-UCB                                               |     36.12 |       40.4295 |                             6.1041 | 2.46s  |
| Thompson Sampling                                           |     35.01 |       40.5420 |                             7.5125 | 0.78s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     34.96 |       40.5786 |                             7.5540 | 0.99s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     34.87 |       40.6461 |                             7.5447 | 1.02s  |
| Garbage In, Reward Out (a=0.10)                             |     33.73 |       42.0945 |                             7.6013 | 1.20s  |
| Perturbed-History Exploration (a=1.1)                       |     33.49 |       42.3004 |                             7.7267 | 0.95s  |
| KL-UCB                                                      |     34.54 |       42.7149 |                             6.2245 | 10.07s |
| EB-TCI                                                      |     30.56 |       42.8317 |                             9.3319 | 0.36s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     32.52 |       43.1108 |                             8.0902 | 1.07s  |
| Non-Parametric Thompson Sampling                            |     33.09 |       43.6865 |                             7.5605 | 4.69s  |
| Vanilla Residual Bootstrap (init=1)                         |     32.88 |       43.7710 |                             7.4509 | 0.27s  |
| Bounded Dirichlet Sampling                                  |     32.79 |       44.7466 |                             7.9659 | 2.76s  |
| Tsallis-INF                                                 |     32.35 |       45.6862 |                             8.4068 | 2.25s  |
| lil' UCB (δ=0.100)                                          |     31.70 |       46.4287 |                             6.7023 | 0.38s  |
| Kullback-Leibler Maillard Sampling                          |     29.69 |       47.8324 |                             8.4744 | 0.65s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     27.45 |       48.1450 |                            10.2207 | 1.04s  |
| Garbage In, Reward Out (a=0.33)                             |     30.11 |       48.1458 |                             8.0648 | 1.29s  |
| PFLA (n=100)                                                |     25.52 |       48.2747 |                            11.5641 | 86.16s |
| ReBoot (r=0.90)                                             |     29.34 |       48.4181 |                             8.4845 | 0.30s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     33.79 |       49.1413 |                             7.5396 | 0.15s  |
| ETC (m=5)                                                   |     21.32 |       50.0278 |                            17.6885 | 0.13s  |
| ReBoot (r=1.00)                                             |     27.89 |       50.9352 |                             8.6898 | 0.27s  |
| ETC (m=20)                                                  |     31.24 |       51.1732 |                             8.6350 | 0.13s  |
| Perturbed-History Exploration (a=2.1)                       |     27.91 |       52.2188 |                             8.4423 | 1.06s  |
| Vanilla Residual Bootstrap (init=5)                         |     28.26 |       53.2834 |                             8.4062 | 0.27s  |
| ETC (m=25)                                                  |     32.18 |       56.3820 |                             8.2546 | 0.14s  |
| lil' UCB (δ=0.010)                                          |     25.83 |       56.9410 |                             8.2814 | 0.39s  |
| Garbage In, Reward Out (a=1.00)                             |     25.12 |       57.7304 |                             9.1152 | 1.24s  |
| Boltzmann-Gumbel Exploration                                |     25.61 |       58.0539 |                             8.8928 | 0.41s  |
| PFLA (n=10)                                                 |     21.31 |       59.7488 |                            12.9092 | 8.87s  |
| ReBoot (r=1.50)                                             |     22.85 |       61.0890 |                             9.6647 | 0.28s  |
| lil' UCB (δ=0.001)                                          |     22.85 |       62.7995 |                             9.1698 | 0.38s  |
| ReBoot (r=1.70)                                             |     21.38 |       64.4112 |                            10.0761 | 0.28s  |
| UCB1                                                        |     20.42 |       68.0927 |                            10.1489 | 0.20s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     24.60 |       68.8686 |                             9.8576 | 0.12s  |
| ReBoot (r=2.10)                                             |     19.16 |       69.7726 |                            10.8419 | 0.28s  |
| ETC (m=3)                                                   |     15.41 |       69.9994 |                            18.3348 | 0.13s  |
| EXP-IX                                                      |     19.28 |       71.2582 |                            11.2795 | 0.59s  |
| Gradient Bandit                                             |     19.00 |       75.4704 |                            12.4808 | 0.49s  |
| Gradient Bandit (with baseline)                             |     18.51 |       77.0723 |                            10.7145 | 0.55s  |
| ETC (m=2)                                                   |     15.27 |       80.4676 |                            18.0151 | 0.10s  |
| PFLA (n=1)                                                  |     10.42 |      100.4073 |                            14.7685 | 1.10s  |
| Random                                                      |     10.01 |      102.0080 |                            15.1748 | 0.02s  |
| CODE (δ=0.050)                                              |     10.00 |      102.0185 |                            14.8649 | 0.41s  |
<!-- END mdsh -->

</details>

### Hard

This experiment uses 10 arms. All arms have a success probability of 0.5, except
for the best arm, which has a success probability of 0.51.

This experiment was taken from the paper describing Boltzmann-Gumbel Exploration.

<details>
<summary>Results</summary>

<!-- `> cat hard.md` -->

<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| POKER (H=100)                                               |     18.10 |        4.0949 |                             0.1700 | 0.35s  |
| Greedy                                                      |     17.00 |        4.1498 |                             0.1100 | 0.09s  |
| POKER (H=10)                                                |     17.00 |        4.1498 |                             0.1100 | 0.34s  |
| POKER (H=1)                                                 |     17.00 |        4.1498 |                             0.1100 | 0.31s  |
| POKER (H=25)                                                |     17.00 |        4.1498 |                             0.1100 | 0.33s  |
| POKER (H=5)                                                 |     17.00 |        4.1498 |                             0.1100 | 0.33s  |
| POKER (H=50)                                                |     17.00 |        4.1499 |                             0.1100 | 0.34s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     16.90 |        4.1552 |                             0.1000 | 0.18s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     16.80 |        4.1598 |                             0.1000 | 0.17s  |
| ϵ-Greedy (ϵ=0.010)                                          |     16.64 |        4.1682 |                             0.1000 | 0.09s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     16.29 |        4.1854 |                             0.1000 | 0.17s  |
| ϵ-Greedy (ϵ=0.020)                                          |     16.25 |        4.1873 |                             0.1000 | 0.12s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     15.26 |        4.2371 |                             0.1700 | 0.19s  |
| ϵ-Greedy (ϵ=0.050)                                          |     15.11 |        4.2447 |                             0.0900 | 0.12s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     14.77 |        4.2614 |                             0.0800 | 0.27s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     14.23 |        4.2887 |                             0.1700 | 0.93s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     14.10 |        4.2951 |                             0.1700 | 7.94s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     14.05 |        4.2973 |                             0.1600 | 0.16s  |
| ϵ-Greedy (ϵ=0.100)                                          |     13.97 |        4.3014 |                             0.0800 | 0.12s  |
| ϵ-Exploring Thompson Sampling                               |     13.74 |        4.3130 |                             0.1100 | 0.18s  |
| Forced Exploration                                          |     13.53 |        4.3235 |                             0.1000 | 0.08s  |
| IRS.FH (H=1)                                                |     13.36 |        4.3318 |                             0.1200 | 1.25s  |
| UCB-DT (γ=0.90)                                             |     13.27 |        4.3365 |                             0.1000 | 5.08s  |
| UCB-DT (γ=0.95)                                             |     13.27 |        4.3365 |                             0.1000 | 5.02s  |
| UCB-DT (γ=1.00)                                             |     13.19 |        4.3406 |                             0.1200 | 3.94s  |
| UCB-DT (γ=0.75)                                             |     13.05 |        4.3474 |                             0.1000 | 5.03s  |
| MOSS-Anytime (α=-0.33)                                      |     13.00 |        4.3502 |                             0.2000 | 0.29s  |
| MOSS-Anytime (α=-0.85)                                      |     12.95 |        4.3526 |                             0.1800 | 0.24s  |
| MOSS-Anytime (α=-0.50)                                      |     12.94 |        4.3532 |                             0.1700 | 0.30s  |
| IRS.FH (H=2)                                                |     12.84 |        4.3582 |                             0.1700 | 1.30s  |
| IRS.FH (H=3)                                                |     12.63 |        4.3687 |                             0.1900 | 1.33s  |
| BayesUCB (δ=0.500)                                          |     12.44 |        4.3782 |                             0.2000 | 0.23s  |
| POKER (H=250)                                               |     12.40 |        4.3802 |                             0.2500 | 0.36s  |
| BayesUCB (δ=0.400)                                          |     12.37 |        4.3813 |                             0.2000 | 0.24s  |
| IRS.FH (H=4)                                                |     12.29 |        4.3857 |                             0.1900 | 1.34s  |
| TS-UCB (100 samples)                                        |     12.17 |        4.3915 |                             0.2500 | 81.68s |
| UCBT                                                        |     12.17 |        4.3916 |                             0.4200 | 0.10s  |
| BayesUCB (δ=0.300)                                          |     12.17 |        4.3917 |                             0.2500 | 0.24s  |
| IRS.FH (H=5)                                                |     12.02 |        4.3990 |                             0.2200 | 1.38s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     11.91 |        4.4043 |                             0.1500 | 0.11s  |
| Bootstrapped Thompson Sampling (J=10)                       |     11.83 |        4.4083 |                             0.1600 | 0.36s  |
| Bootstrapped Thompson Sampling (J=500)                      |     11.80 |        4.4101 |                             0.3400 | 4.44s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     11.78 |        4.4109 |                             0.3400 | 8.54s  |
| Bootstrapped Thompson Sampling (J=100)                      |     11.76 |        4.4118 |                             0.3100 | 1.10s  |
| EB-TCI                                                      |     11.56 |        4.4218 |                             0.4400 | 0.36s  |
| WR-SDA                                                      |     11.52 |        4.4238 |                             0.3200 | 1.53s  |
| BayesUCB (δ=0.200)                                          |     11.51 |        4.4245 |                             0.2300 | 0.24s  |
| IRS.FH (H=10)                                               |     11.46 |        4.4270 |                             0.2400 | 1.45s  |
| TS-UCB (10 samples)                                         |     11.46 |        4.4271 |                             0.2500 | 8.06s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     11.45 |        4.4276 |                             0.2600 | 5.85s  |
| Vanilla Residual Bootstrap (init=0)                         |     11.42 |        4.4292 |                             0.3500 | 0.21s  |
| CODE (δ=0.900)                                              |     11.39 |        4.4305 |                             0.4900 | 0.42s  |
| ReBoot (r=0.25)                                             |     11.38 |        4.4311 |                             0.3500 | 0.24s  |
| BayesUCB (δ=0.900)                                          |     11.37 |        4.4314 |                             0.1200 | 0.23s  |
| ReBoot (r=0.50)                                             |     11.34 |        4.4329 |                             0.3800 | 0.29s  |
| TS-UCB (1 samples)                                          |     11.21 |        4.4395 |                             0.2400 | 0.99s  |
| CODE (δ=0.990)                                              |     11.21 |        4.4397 |                             0.1200 | 0.39s  |
| Optimistic Thompson Sampling                                |     11.20 |        4.4399 |                             0.3000 | 1.05s  |
| Garbage In, Reward Out (a=0.10)                             |     11.16 |        4.4418 |                             0.3400 | 1.23s  |
| BayesUCB (δ=0.100)                                          |     11.16 |        4.4421 |                             0.2400 | 0.20s  |
| Non-Parametric Thompson Sampling                            |     11.16 |        4.4422 |                             0.3400 | 4.63s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     11.15 |        4.4425 |                             0.3400 | 1.02s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     11.15 |        4.4426 |                             0.3300 | 0.98s  |
| Thompson Sampling                                           |     11.14 |        4.4429 |                             0.3300 | 0.79s  |
| IRS.FH (H=25)                                               |     11.14 |        4.4431 |                             0.2600 | 2.06s  |
| Perturbed-History Exploration (a=1.1)                       |     11.13 |        4.4433 |                             0.3600 | 0.97s  |
| Multiplier Bootstrap-based Exploration                      |     11.12 |        4.4439 |                             0.3100 | 5.83s  |
| Vanilla Residual Bootstrap (init=1)                         |     11.11 |        4.4443 |                             0.3500 | 0.27s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     11.09 |        4.4454 |                             0.4000 | 1.09s  |
| Garbage In, Reward Out (a=0.33)                             |     11.04 |        4.4480 |                             0.3800 | 1.20s  |
| Tsallis-INF                                                 |     11.01 |        4.4497 |                             0.2700 | 2.34s  |
| KL-UCB                                                      |     10.99 |        4.4505 |                             0.2800 | 9.72s  |
| ReBoot (r=0.90)                                             |     10.94 |        4.4528 |                             0.3800 | 0.29s  |
| Kullback-Leibler Maillard Sampling                          |     10.91 |        4.4544 |                             0.3500 | 0.66s  |
| Perturbed-History Exploration (a=2.1)                       |     10.89 |        4.4557 |                             0.3400 | 1.05s  |
| Vanilla Residual Bootstrap (init=5)                         |     10.85 |        4.4574 |                             0.2700 | 0.26s  |
| Hellinger-UCB                                               |     10.85 |        4.4575 |                             0.2800 | 2.51s  |
| lil' UCB (δ=0.100)                                          |     10.85 |        4.4575 |                             0.2600 | 0.37s  |
| ReBoot (r=1.00)                                             |     10.84 |        4.4578 |                             0.3500 | 0.29s  |
| Bounded Dirichlet Sampling                                  |     10.83 |        4.4586 |                             0.3100 | 2.50s  |
| UCB1-Tuned                                                  |     10.74 |        4.4632 |                             0.2400 | 0.30s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     10.72 |        4.4641 |                             0.3100 | 1.11s  |
| lil' UCB (δ=0.010)                                          |     10.70 |        4.4651 |                             0.2200 | 0.38s  |
| Boltzmann-Gumbel Exploration                                |     10.67 |        4.4663 |                             0.2700 | 0.42s  |
| Garbage In, Reward Out (a=1.00)                             |     10.66 |        4.4669 |                             0.2600 | 1.23s  |
| lil' UCB (δ=0.001)                                          |     10.54 |        4.4730 |                             0.2000 | 0.36s  |
| PFLA (n=10)                                                 |     10.52 |        4.4740 |                             0.3800 | 8.85s  |
| ReBoot (r=1.50)                                             |     10.49 |        4.4756 |                             0.2100 | 0.29s  |
| PFLA (n=100)                                                |     10.48 |        4.4759 |                             0.2400 | 84.92s |
| ReBoot (r=1.70)                                             |     10.40 |        4.4798 |                             0.1800 | 0.28s  |
| EXP-IX                                                      |     10.36 |        4.4822 |                             0.1600 | 0.58s  |
| ReBoot (r=2.10)                                             |     10.29 |        4.4854 |                             0.1400 | 0.29s  |
| ETC (m=25)                                                  |     10.27 |        4.4863 |                             0.0000 | 0.15s  |
| Gradient Bandit                                             |     10.27 |        4.4866 |                             0.1300 | 0.50s  |
| UCB1                                                        |     10.23 |        4.4883 |                             0.1600 | 0.17s  |
| Gradient Bandit (with baseline)                             |     10.23 |        4.4887 |                             0.1100 | 0.49s  |
| ETC (m=5)                                                   |     10.11 |        4.4943 |                             0.0000 | 0.14s  |
| ETC (m=20)                                                  |     10.11 |        4.4946 |                             0.0000 | 0.14s  |
| ETC (m=2)                                                   |     10.04 |        4.4982 |                             0.4300 | 0.10s  |
| ETC (m=3)                                                   |     10.04 |        4.4982 |                             0.4300 | 0.13s  |
| Random                                                      |     10.02 |        4.4992 |                             0.0500 | 0.01s  |
| PFLA (n=1)                                                  |     10.01 |        4.4993 |                             0.0100 | 1.10s  |
| CODE (δ=0.050)                                              |     10.00 |        4.5000 |                             0.0000 | 0.42s  |
| ETC (m=10)                                                  |      9.94 |        4.5030 |                             0.0000 | 0.14s  |
<!-- END mdsh -->

</details>

### Beta

This experiment uses 10 arms. The arm means are sampled from a Beta(1, 8) distribution.

This experiment was taken from the paper *Multiplier Bootstrap-based Exploration*.

<details>
<summary>Results</summary>

<!-- `> cat beta.md` -->

<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| BayesUCB (δ=0.900)                                          |     57.80 |       20.8173 |                             5.4766 | 0.26s  |
| IRS.FH (H=1)                                                |     55.74 |       21.8777 |                             5.8971 | 1.39s  |
| MOSS-Anytime (α=-0.85)                                      |     54.95 |       22.2898 |                             5.6517 | 0.25s  |
| UCB-DT (γ=0.75)                                             |     54.64 |       22.4071 |                             6.1492 | 4.37s  |
| UCB-DT (γ=0.90)                                             |     54.45 |       22.4627 |                             6.1571 | 4.32s  |
| UCB-DT (γ=0.95)                                             |     54.39 |       22.4968 |                             6.1852 | 4.29s  |
| UCB-DT (γ=1.00)                                             |     53.32 |       22.6778 |                             7.3649 | 4.44s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     56.94 |       22.9406 |                             7.1140 | 20.69s |
| IRS.FH (H=2)                                                |     56.33 |       23.0910 |                             5.0660 | 1.35s  |
| CODE (δ=0.990)                                              |     51.11 |       23.5974 |                             9.3932 | 0.40s  |
| MOSS-Anytime (α=-0.50)                                      |     56.24 |       24.1465 |                             4.0881 | 0.28s  |
| BayesUCB (δ=0.500)                                          |     56.42 |       24.2684 |                             5.5883 | 0.26s  |
| IRS.FH (H=3)                                                |     55.61 |       24.5294 |                             4.7675 | 1.42s  |
| TS-UCB (100 samples)                                        |     56.12 |       24.7437 |                             4.2837 | 84.53s |
| ReBoot (r=0.25)                                             |     52.26 |       24.7586 |                             8.6759 | 0.24s  |
| IRS.FH (H=4)                                                |     55.19 |       25.4645 |                             4.9019 | 1.39s  |
| BayesUCB (δ=0.400)                                          |     55.31 |       25.7435 |                             5.7135 | 0.27s  |
| MOSS-Anytime (α=-0.33)                                      |     54.80 |       26.1464 |                             4.2098 | 0.28s  |
| IRS.FH (H=5)                                                |     54.72 |       26.2315 |                             4.9551 | 1.41s  |
| TS-UCB (10 samples)                                         |     54.96 |       26.2734 |                             4.2721 | 8.89s  |
| BayesUCB (δ=0.300)                                          |     53.93 |       27.5129 |                             5.7981 | 0.27s  |
| IRS.FH (H=10)                                               |     53.19 |       28.6872 |                             5.2676 | 1.48s  |
| UCBT                                                        |     47.49 |       28.8558 |                             8.0049 | 0.11s  |
| ReBoot (r=0.50)                                             |     51.44 |       28.9633 |                             6.3791 | 0.28s  |
| TS-UCB (1 samples)                                          |     52.69 |       29.2908 |                             4.9082 | 1.09s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     47.60 |       29.4661 |                             9.1640 | 0.22s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     47.35 |       29.8509 |                             9.2614 | 1.01s  |
| BayesUCB (δ=0.200)                                          |     51.93 |       29.8533 |                             5.8074 | 0.27s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     47.30 |       29.8928 |                             9.1374 | 8.58s  |
| Hellinger-UCB                                               |     50.41 |       30.1850 |                             5.4750 | 2.50s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     45.66 |       30.9426 |                            10.3885 | 0.28s  |
| Bootstrapped Thompson Sampling (J=10)                       |     49.88 |       31.1623 |                             6.5576 | 0.38s  |
| Forced Exploration                                          |     48.86 |       31.4112 |                             9.0715 | 0.09s  |
| IRS.FH (H=25)                                               |     50.57 |       32.0301 |                             5.6474 | 1.63s  |
| Multiplier Bootstrap-based Exploration                      |     49.17 |       32.4139 |                             6.0942 | 5.86s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     42.88 |       32.7340 |                            12.0469 | 0.18s  |
| ϵ-Exploring Thompson Sampling                               |     44.38 |       33.2239 |                            12.5400 | 0.17s  |
| ϵ-Greedy (ϵ=0.100)                                          |     44.10 |       33.2831 |                            11.8153 | 0.13s  |
| BayesUCB (δ=0.100)                                          |     48.69 |       33.6207 |                             5.8840 | 0.23s  |
| ϵ-Greedy (ϵ=0.050)                                          |     42.23 |       33.7998 |                            13.3609 | 0.14s  |
| UCB1-Tuned                                                  |     48.22 |       34.0173 |                             5.5690 | 0.33s  |
| Bootstrapped Thompson Sampling (J=100)                      |     47.17 |       34.7044 |                             6.3147 | 1.08s  |
| Garbage In, Reward Out (a=0.10)                             |     46.53 |       35.1309 |                             6.4203 | 0.98s  |
| Bootstrapped Thompson Sampling (J=500)                      |     46.87 |       35.1931 |                             6.3344 | 4.45s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     46.77 |       35.2492 |                             6.3528 | 8.55s  |
| Vanilla Residual Bootstrap (init=1)                         |     46.87 |       35.3194 |                             6.1483 | 0.35s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     40.31 |       35.3717 |                            14.8777 | 0.19s  |
| Optimistic Thompson Sampling                                |     47.14 |       35.7522 |                             6.0706 | 1.16s  |
| ϵ-Greedy (ϵ=0.020)                                          |     39.94 |       35.9324 |                            16.4079 | 0.14s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     39.46 |       36.2891 |                            16.3213 | 0.18s  |
| Vanilla Residual Bootstrap (init=0)                         |     39.16 |       36.6560 |                            16.2068 | 0.23s  |
| ETC (m=5)                                                   |     39.97 |       37.5465 |                            17.0296 | 0.14s  |
| Thompson Sampling                                           |     45.10 |       37.7381 |                             6.5241 | 0.81s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     45.09 |       37.7390 |                             6.4866 | 1.08s  |
| ϵ-Greedy (ϵ=0.010)                                          |     38.41 |       37.7394 |                            18.4671 | 0.11s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     44.99 |       37.8316 |                             6.5136 | 1.11s  |
| KL-UCB                                                      |     44.75 |       37.9754 |                             5.7666 | 9.28s  |
| ETC (m=10)                                                  |     40.33 |       38.9869 |                            13.6763 | 0.14s  |
| Non-Parametric Thompson Sampling                            |     43.90 |       39.3507 |                             6.7160 | 4.59s  |
| Greedy                                                      |     36.66 |       39.9099 |                            21.7087 | 0.10s  |
| Bounded Dirichlet Sampling                                  |     43.54 |       39.9645 |                             6.6454 | 2.70s  |
| CODE (δ=0.900)                                              |     40.61 |       40.2050 |                            13.2482 | 0.44s  |
| ReBoot (r=0.90)                                             |     42.54 |       40.7881 |                             7.1968 | 0.28s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     41.59 |       40.8984 |                             7.1496 | 1.12s  |
| WR-SDA                                                      |     36.73 |       41.0443 |                            19.6162 | 1.97s  |
| POKER (H=100)                                               |     36.39 |       41.3770 |                            22.1022 | 0.28s  |
| POKER (H=250)                                               |     36.40 |       41.3832 |                            21.9572 | 0.28s  |
| POKER (H=50)                                                |     36.31 |       41.4064 |                            22.3819 | 0.28s  |
| POKER (H=25)                                                |     36.24 |       41.4405 |                            22.6058 | 0.29s  |
| Kullback-Leibler Maillard Sampling                          |     40.83 |       41.4463 |                             7.5405 | 0.64s  |
| POKER (H=10)                                                |     36.05 |       41.5399 |                            23.0560 | 0.28s  |
| POKER (H=5)                                                 |     35.92 |       41.6486 |                            23.3861 | 0.29s  |
| POKER (H=1)                                                 |     35.72 |       41.8819 |                            23.9184 | 0.26s  |
| Perturbed-History Exploration (a=1.1)                       |     40.79 |       42.7866 |                             7.3646 | 0.90s  |
| ReBoot (r=1.00)                                             |     40.65 |       43.3432 |                             7.6618 | 0.28s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     41.58 |       44.0842 |                            11.9547 | 0.16s  |
| Garbage In, Reward Out (a=0.33)                             |     38.75 |       45.1922 |                             7.8091 | 1.09s  |
| ETC (m=3)                                                   |     33.51 |       45.7840 |                            28.1017 | 0.14s  |
| ETC (m=20)                                                  |     37.94 |       47.1134 |                            13.4466 | 0.14s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     33.63 |       48.2505 |                             9.6052 | 1.11s  |
| lil' UCB (δ=0.100)                                          |     36.34 |       48.6046 |                             7.3285 | 0.38s  |
| ETC (m=25)                                                  |     37.82 |       51.7141 |                            13.9357 | 0.15s  |
| ETC (m=2)                                                   |     29.53 |       53.1694 |                            28.6333 | 0.11s  |
| ReBoot (r=1.50)                                             |     33.20 |       53.6329 |                            10.8113 | 0.28s  |
| Perturbed-History Exploration (a=2.1)                       |     32.69 |       53.7141 |                             9.5801 | 1.10s  |
| Tsallis-INF                                                 |     32.39 |       54.7917 |                            11.3371 | 2.50s  |
| Vanilla Residual Bootstrap (init=5)                         |     31.29 |       56.3702 |                            10.3022 | 0.27s  |
| ReBoot (r=1.70)                                             |     30.99 |       56.8129 |                            12.0371 | 0.28s  |
| Garbage In, Reward Out (a=1.00)                             |     29.48 |       58.3347 |                            11.5697 | 1.30s  |
| Boltzmann-Gumbel Exploration                                |     29.89 |       58.4917 |                            11.5794 | 0.44s  |
| lil' UCB (δ=0.010)                                          |     29.25 |       58.7242 |                            11.2953 | 0.38s  |
| EB-TCI                                                      |     24.42 |       59.0388 |                            22.7179 | 0.33s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     30.85 |       61.5675 |                            16.7498 | 0.12s  |
| ReBoot (r=2.10)                                             |     27.57 |       61.9376 |                            14.1671 | 0.28s  |
| PFLA (n=100)                                                |     22.78 |       64.3650 |                            18.6443 | 86.49s |
| lil' UCB (δ=0.001)                                          |     25.41 |       64.5631 |                            14.7753 | 0.39s  |
| UCB1                                                        |     22.31 |       69.6096 |                            17.0817 | 0.21s  |
| PFLA (n=10)                                                 |     18.26 |       73.4227 |                            21.0101 | 8.77s  |
| Gradient Bandit                                             |     20.28 |       74.1103 |                            17.5927 | 0.48s  |
| Gradient Bandit (with baseline)                             |     19.93 |       74.7782 |                            17.8747 | 0.50s  |
| EXP-IX                                                      |     17.71 |       77.5879 |                            20.1208 | 0.61s  |
| PFLA (n=1)                                                  |     10.45 |       92.1317 |                            25.7794 | 1.10s  |
| Random                                                      |      9.99 |       93.1436 |                            26.0904 | 0.03s  |
| CODE (δ=0.050)                                              |     10.00 |       93.1468 |                            25.9588 | 0.42s  |
<!-- END mdsh -->

</details>

### Reverse Beta

This experiment uses 10 arms. The arm means are sampled from a Beta(8, 1) distribution.

I added this to see which algorithms are affected by rewards close to 1 instead of close to 0.

<details>
<summary>Results</summary>

<!-- `> cat reverse_beta.md` -->

<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| TS-UCB (100 samples)                                        |     57.85 |        6.9470 |                             2.3223 | 84.38s |
| TS-UCB (10 samples)                                         |     57.75 |        7.2685 |                             2.1360 | 8.58s  |
| POKER (H=5)                                                 |     55.21 |        7.8042 |                             1.8429 | 0.35s  |
| TS-UCB (1 samples)                                          |     57.78 |        7.8597 |                             1.9439 | 1.03s  |
| POKER (H=1)                                                 |     56.13 |        8.0894 |                             1.7428 | 0.34s  |
| POKER (H=10)                                                |     48.12 |        8.3258 |                             2.2912 | 0.35s  |
| UCB-DT (γ=1.00)                                             |     55.66 |        8.4709 |                             1.4837 | 4.70s  |
| IRS.FH (H=2)                                                |     54.85 |        8.4788 |                             1.5053 | 1.40s  |
| ϵ-Exploring TS-UCB (100 samples)                            |     54.56 |        8.4801 |                             1.5113 | 8.39s  |
| IRS.FH (H=3)                                                |     55.12 |        8.4888 |                             1.5141 | 1.67s  |
| ϵ-Exploring TS-UCB (10 samples)                             |     54.54 |        8.5219 |                             1.5144 | 0.95s  |
| IRS.FH (H=1)                                                |     54.33 |        8.5340 |                             1.5095 | 1.20s  |
| IRS.FH (H=4)                                                |     55.12 |        8.5479 |                             1.5428 | 1.39s  |
| UCB-DT (γ=0.90)                                             |     55.84 |        8.5612 |                             1.5143 | 4.86s  |
| ϵ-Exploring TS-UCB (1 samples)                              |     54.46 |        8.5696 |                             1.5205 | 0.20s  |
| UCB-DT (γ=0.95)                                             |     55.76 |        8.5784 |                             1.5121 | 4.99s  |
| IRS.FH (H=5)                                                |     55.41 |        8.5928 |                             1.5664 | 1.70s  |
| Greedy                                                      |     53.82 |        8.6471 |                             1.5273 | 0.09s  |
| UCB-DT (γ=0.75)                                             |     55.91 |        8.6739 |                             1.5558 | 5.09s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     53.57 |        8.8897 |                             1.5610 | 0.18s  |
| IRS.FH (H=10)                                               |     55.40 |        8.9571 |                             1.7240 | 1.46s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     53.45 |        9.0268 |                             1.5815 | 0.18s  |
| BayesUCB (δ=0.200)                                          |     43.36 |        9.0488 |                             4.8228 | 0.25s  |
| ϵ-Greedy (ϵ=0.010)                                          |     53.34 |        9.0561 |                             1.5947 | 0.09s  |
| Optimistic Thompson Sampling                                |     54.95 |        9.3835 |                             3.1222 | 0.99s  |
| ϵ-Greedy (ϵ=0.020)                                          |     52.84 |        9.4814 |                             1.6900 | 0.13s  |
| CODE (δ=0.990)                                              |     48.81 |        9.4822 |                             1.7486 | 0.42s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     52.78 |        9.6599 |                             1.7204 | 0.17s  |
| IRS.FH (H=25)                                               |     54.09 |       10.0328 |                             2.1203 | 1.60s  |
| WR-SDA                                                      |     52.23 |       10.2123 |                             2.7845 | 0.80s  |
| POKER (H=50)                                                |     41.34 |       10.6033 |                             2.7010 | 0.36s  |
| POKER (H=25)                                                |     41.02 |       10.6190 |                             2.7176 | 0.34s  |
| POKER (H=100)                                               |     41.56 |       10.6923 |                             2.7973 | 0.35s  |
| ϵ-Greedy (ϵ=0.050)                                          |     51.57 |       10.7107 |                             1.9386 | 0.13s  |
| POKER (H=250)                                               |     42.10 |       10.8128 |                             2.8381 | 0.35s  |
| ϵ-Exploring Thompson Sampling                               |     45.15 |       10.8883 |                             4.0500 | 0.16s  |
| MOSS-Anytime (α=-0.85)                                      |     50.92 |       11.2098 |                             1.9147 | 0.24s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     50.84 |       11.5088 |                             2.0720 | 0.29s  |
| KL-UCB                                                      |     51.49 |       11.6751 |                             3.5785 | 7.97s  |
| Thompson Sampling                                           |     48.51 |       12.4396 |                             2.7769 | 0.87s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     37.00 |       12.6025 |                             4.1618 | 19.25s |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     47.58 |       12.6638 |                             2.7907 | 1.07s  |
| ϵ-Greedy (ϵ=0.100)                                          |     49.34 |       12.7974 |                             2.3308 | 0.13s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     45.49 |       13.1710 |                             2.8249 | 1.13s  |
| Non-Parametric Thompson Sampling                            |     47.46 |       13.6038 |                             4.3455 | 4.59s  |
| BayesUCB (δ=0.300)                                          |     26.75 |       13.8625 |                             8.5920 | 0.24s  |
| Forced Exploration                                          |     48.30 |       13.9900 |                             2.5181 | 0.10s  |
| Bounded Dirichlet Sampling                                  |     45.58 |       14.5418 |                             4.6561 | 2.45s  |
| PFLA (n=100)                                                |     32.03 |       14.6253 |                             5.1971 | 94.98s |
| BayesUCB (δ=0.100)                                          |     37.27 |       15.0673 |                             2.3660 | 0.21s  |
| Kullback-Leibler Maillard Sampling                          |     43.53 |       15.1294 |                             5.1731 | 0.58s  |
| MOSS-Anytime (α=-0.50)                                      |     44.06 |       15.3933 |                             2.1697 | 0.27s  |
| Hellinger-UCB                                               |     43.81 |       15.5306 |                             5.4955 | 1.59s  |
| MOSS-Anytime (α=-0.33)                                      |     40.99 |       17.0540 |                             2.2724 | 0.27s  |
| UCBT                                                        |     32.33 |       18.1863 |                             6.0728 | 0.12s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     23.97 |       18.9613 |                             5.2597 | 1.19s  |
| BayesUCB (δ=0.400)                                          |     19.76 |       19.4237 |                            12.4626 | 0.25s  |
| EB-TCI                                                      |     35.93 |       19.7395 |                             5.2415 | 0.35s  |
| ReBoot (r=0.25)                                             |     34.89 |       19.9697 |                             3.1894 | 0.29s  |
| PFLA (n=10)                                                 |     26.61 |       21.4150 |                             6.9893 | 9.46s  |
| Vanilla Residual Bootstrap (init=0)                         |     33.47 |       21.5207 |                             3.2212 | 0.24s  |
| Multiplier Bootstrap-based Exploration                      |     28.45 |       22.2710 |                             3.5416 | 5.86s  |
| ETC (m=20)                                                  |     33.55 |       22.3233 |                             4.2529 | 0.14s  |
| ETC (m=10)                                                  |     27.09 |       22.3539 |                             6.4168 | 0.15s  |
| ReBoot (r=0.50)                                             |     30.87 |       22.5161 |                             3.8147 | 0.33s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     38.74 |       22.6530 |                             4.3599 | 0.16s  |
| UCB1-Tuned                                                  |     25.07 |       22.9077 |                             3.4824 | 0.43s  |
| Vanilla Residual Bootstrap (init=1)                         |     31.19 |       23.1578 |                             3.3412 | 0.29s  |
| Tsallis-INF                                                 |     26.30 |       23.2635 |                             4.3108 | 2.65s  |
| Vanilla Residual Bootstrap (init=5)                         |     30.48 |       23.8825 |                             3.5154 | 0.28s  |
| Garbage In, Reward Out (a=0.10)                             |     26.82 |       23.9510 |                             3.8778 | 0.99s  |
| Perturbed-History Exploration (a=1.1)                       |     24.17 |       24.8624 |                             4.3134 | 1.21s  |
| BayesUCB (δ=0.500)                                          |     15.86 |       25.3034 |                            16.2815 | 0.26s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     12.50 |       26.5896 |                             8.8139 | 1.15s  |
| ETC (m=25)                                                  |     28.64 |       27.0247 |                             5.2417 | 0.15s  |
| CODE (δ=0.900)                                              |     16.26 |       27.7259 |                             4.4425 | 0.48s  |
| Garbage In, Reward Out (a=0.33)                             |     21.22 |       28.0093 |                             4.7583 | 1.23s  |
| Bootstrapped Thompson Sampling (J=500)                      |     17.05 |       28.0954 |                            16.5475 | 4.34s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     16.11 |       28.1867 |                            16.6249 | 8.47s  |
| ReBoot (r=0.90)                                             |     24.08 |       28.2376 |                             5.0547 | 0.32s  |
| lil' UCB (δ=0.100)                                          |     19.19 |       28.5694 |                             4.7509 | 0.38s  |
| Bootstrapped Thompson Sampling (J=100)                      |     15.82 |       29.0489 |                            16.7117 | 1.13s  |
| ReBoot (r=1.00)                                             |     22.53 |       29.7884 |                             5.3791 | 0.33s  |
| Bootstrapped Thompson Sampling (J=10)                       |     15.12 |       30.0861 |                            17.7177 | 0.36s  |
| Perturbed-History Exploration (a=2.1)                       |     18.72 |       30.3983 |                             5.2058 | 1.18s  |
| lil' UCB (δ=0.010)                                          |     16.72 |       32.2288 |                             5.5208 | 0.39s  |
| Garbage In, Reward Out (a=1.00)                             |     17.26 |       32.4632 |                             5.6672 | 1.28s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     27.51 |       32.6383 |                             6.3517 | 0.12s  |
| Boltzmann-Gumbel Exploration                                |     17.44 |       32.7460 |                             5.6438 | 0.42s  |
| lil' UCB (δ=0.001)                                          |     15.51 |       34.2797 |                             5.8524 | 0.35s  |
| EXP-IX                                                      |     15.62 |       34.8327 |                             6.2311 | 0.61s  |
| ReBoot (r=1.50)                                             |     18.20 |       35.2644 |                             6.5573 | 0.32s  |
| UCB1                                                        |     14.55 |       36.1248 |                             6.3580 | 0.27s  |
| ReBoot (r=1.70)                                             |     17.25 |       36.7828 |                             6.9301 | 0.32s  |
| BayesUCB (δ=0.900)                                          |     12.29 |       38.7864 |                            24.1887 | 0.25s  |
| ReBoot (r=2.10)                                             |     15.90 |       39.2124 |                             7.5247 | 0.36s  |
| Gradient Bandit                                             |     13.72 |       39.5229 |                             8.1141 | 0.52s  |
| Gradient Bandit (with baseline)                             |     13.15 |       40.8926 |                             7.4944 | 0.48s  |
| ETC (m=5)                                                   |     12.36 |       41.7571 |                             9.1900 | 0.16s  |
| ETC (m=3)                                                   |     12.03 |       43.5920 |                             9.6906 | 0.15s  |
| ETC (m=2)                                                   |     11.03 |       45.2564 |                             9.3287 | 0.11s  |
| PFLA (n=1)                                                  |     10.44 |       48.2947 |                             9.7754 | 1.23s  |
| CODE (δ=0.050)                                              |     10.00 |       49.2639 |                             9.8811 | 0.44s  |
| Random                                                      |      9.99 |       49.2870 |                            10.0029 | 0.03s  |
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
