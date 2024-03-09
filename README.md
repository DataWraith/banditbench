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
- [Non-Parametric Thompson Sampling](https://proceedings.mlr.press/v117/riou20a.html)
- Optimistic Thompson Sampling
- [Satisficing Thompson Sampling](https://arxiv.org/abs/1704.09028)
- [Thompson Sampling with Virtual Helping Agents (C3)](https://arxiv.org/abs/2209.08197)
- Thompson Sampling

### Upper Confidence Bound-based

- [KL-UCB](https://arxiv.org/abs/1102.2490)
- [lil' UCB](https://arxiv.org/abs/1312.7308)
- [UCB1](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [UCB1-Tuned](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [UCB-DT](https://arxiv.org/abs/2110.02690)

### Other

- [Boltzmann-Gumbel Exploration](https://arxiv.org/abs/1705.10257)
- [EB-TCI](https://arxiv.org/abs/2206.05979)
- [Forced Exploration](https://arxiv.org/abs/2312.07285)
- [Gradient Bandit](https://arxiv.org/abs/2402.17235)
- [Kullback-Leibler Maillard Sampling](https://arxiv.org/abs/2304.14989)
- [Tsallis-INF](https://arxiv.org/abs/1807.07623)
- [TS-UCB](https://arxiv.org/abs/2006.06372)

## Experiments

### Uniform

This experiment uses 10 arms, with the means sampled uniformly from the interval
[0, 1]. This is a relatively easy instance, because there is likely to be a
single best arm that is easy to find. This is reflected in the %-Optimal column,
where the best algorithms reach over 2/3 pull rate of the optimal arm.

<details>
<summary>Results</summary>

<!-- `> cat uniform.md` -->
<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| Bootstrapped Thompson Sampling (J=500)                      |     81.55 |       12.3112 |                             1.6871 | 4.25s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     81.44 |       12.6758 |                             1.7121 | 8.28s  |
| TS-UCB (100 samples)                                        |     72.43 |       17.4061 |                             3.2706 | 64.83s |
| TS-UCB (10 samples)                                         |     72.88 |       17.8546 |                             3.5976 | 6.70s  |
| UCB-DT (γ=1.00)                                             |     70.73 |       18.2706 |                             2.5295 | 2.40s  |
| UCB-DT (γ=0.90)                                             |     73.02 |       18.3178 |                             2.4600 | 2.58s  |
| UCB-DT (γ=0.95)                                             |     72.98 |       18.3323 |                             2.4505 | 2.50s  |
| UCB-DT (γ=0.75)                                             |     72.98 |       18.3605 |                             2.4852 | 2.45s  |
| Greedy                                                      |     67.48 |       19.7483 |                             2.4973 | 0.07s  |
| TS-UCB (1 samples)                                          |     72.28 |       19.9767 |                             5.3785 | 0.70s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     63.36 |       21.1298 |                             6.2710 | 24.55s |
| ETC (m=2)                                                   |     67.37 |       21.4042 |                             2.4254 | 0.08s  |
| ETC (m=3)                                                   |     67.83 |       23.0491 |                             2.6583 | 0.10s  |
| WR-SDA                                                      |     67.66 |       23.8199 |                             5.0460 | 1.56s  |
| Optimistic Thompson Sampling                                |     69.69 |       25.4924 |                             7.1978 | 0.89s  |
| ETC (m=5)                                                   |     69.09 |       27.4454 |                             3.4486 | 0.11s  |
| ϵ-Exploring Thompson Sampling                               |     64.31 |       27.5471 |                             8.9868 | 0.13s  |
| Thompson Sampling                                           |     67.00 |       28.9445 |                             7.1632 | 0.68s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     66.88 |       29.0225 |                             7.0900 | 0.83s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     66.49 |       29.3398 |                             6.9895 | 0.95s  |
| KL-UCB                                                      |     67.57 |       29.6845 |                             7.4867 | 7.49s  |
| ReBoot (r=0.25)                                             |     61.92 |       30.3897 |                             5.2935 | 0.19s  |
| UCB1-Tuned                                                  |     62.81 |       31.7769 |                             3.6345 | 0.23s  |
| Bootstrapped Thompson Sampling (J=100)                      |     51.85 |       32.8060 |                            13.5494 | 1.04s  |
| Vanilla Residual Bootstrap (init=0)                         |     60.73 |       33.1690 |                             5.4193 | 0.16s  |
| Non-Parametric Thompson Sampling                            |     64.59 |       33.8504 |                             7.0679 | 4.33s  |
| ReBoot (r=0.50)                                             |     59.20 |       34.1884 |                             5.8990 | 0.26s  |
| Bounded Dirichlet Sampling                                  |     64.70 |       34.2376 |                             7.1518 | 2.32s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     58.20 |       34.9791 |                             6.9401 | 0.99s  |
| Multiplier Bootstrap-based Exploration                      |     61.60 |       36.1880 |                             4.1319 | 5.63s  |
| Kullback-Leibler Maillard Sampling                          |     60.53 |       37.5467 |                             8.4138 | 0.51s  |
| Perturbed-History Exploration (a=1.1)                       |     57.78 |       37.8970 |                             5.6488 | 0.73s  |
| Garbage In, Reward Out (a=0.10)                             |     58.49 |       38.7874 |                             5.2714 | 0.86s  |
| Bootstrapped Thompson Sampling (J=10)                       |     45.02 |       39.1305 |                            19.4246 | 0.38s  |
| Vanilla Residual Bootstrap (init=1)                         |     60.28 |       40.6878 |                             4.7500 | 0.20s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     44.92 |       44.1840 |                            10.6738 | 0.98s  |
| ETC (m=10)                                                  |     65.57 |       44.8172 |                             6.1355 | 0.11s  |
| lil' UCB (δ=0.100)                                          |     52.87 |       44.9486 |                             5.5879 | 0.29s  |
| Tsallis-INF                                                 |     55.26 |       46.5441 |                             5.8549 | 0.99s  |
| Forced Exploration                                          |     63.74 |       46.7869 |                             6.0952 | 0.06s  |
| ReBoot (r=0.90)                                             |     52.97 |       47.4265 |                             6.6595 | 0.28s  |
| Garbage In, Reward Out (a=0.33)                             |     52.54 |       49.4032 |                             5.5249 | 1.07s  |
| Vanilla Residual Bootstrap (init=5)                         |     56.46 |       50.8622 |                             6.0489 | 0.22s  |
| ReBoot (r=1.00)                                             |     50.62 |       52.0364 |                             6.7317 | 0.26s  |
| EB-TCI                                                      |     42.95 |       56.0202 |                            16.1098 | 0.30s  |
| Perturbed-History Exploration (a=2.1)                       |     48.19 |       56.7164 |                             6.0494 | 0.89s  |
| lil' UCB (δ=0.010)                                          |     44.60 |       62.4201 |                             6.5913 | 0.30s  |
| Garbage In, Reward Out (a=1.00)                             |     43.69 |       66.7268 |                             7.0150 | 1.09s  |
| Boltzmann-Gumbel Exploration                                |     44.52 |       69.1820 |                             6.7076 | 0.42s  |
| ReBoot (r=1.50)                                             |     41.00 |       72.4983 |                             8.1316 | 0.25s  |
| lil' UCB (δ=0.001)                                          |     39.59 |       74.2236 |                             8.0274 | 0.26s  |
| ReBoot (r=1.70)                                             |     37.90 |       79.8938 |                             9.0403 | 0.23s  |
| UCB1                                                        |     34.84 |       87.3965 |                            10.1205 | 0.15s  |
| ReBoot (r=2.10)                                             |     32.69 |       93.3431 |                            10.7795 | 0.25s  |
| Gradient Bandit                                             |     30.56 |      111.1047 |                            17.4381 | 0.37s  |
| Gradient Bandit (with baseline)                             |     31.78 |      114.0673 |                            11.6366 | 0.42s  |
| Random                                                      |      9.99 |      205.0580 |                            30.3100 | 0.02s  |
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
| Bootstrapped Thompson Sampling (J=500)                      |     60.67 |       20.0135 |                            10.8059 | 4.46s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     60.81 |       20.0744 |                            10.7234 | 8.74s  |
| UCB-DT (γ=0.90)                                             |     44.11 |       25.7379 |                             7.1522 | 2.55s  |
| UCB-DT (γ=0.95)                                             |     44.07 |       25.7444 |                             7.1627 | 2.56s  |
| UCB-DT (γ=0.75)                                             |     44.20 |       25.7518 |                             7.1508 | 2.53s  |
| ETC (m=5)                                                   |     44.22 |       25.9861 |                             6.0215 | 0.12s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     44.83 |       26.7704 |                             8.7872 | 12.11s |
| ETC (m=3)                                                   |     42.10 |       26.9119 |                             7.3339 | 0.11s  |
| TS-UCB (100 samples)                                        |     44.83 |       27.4483 |                             6.6267 | 68.59s |
| Bootstrapped Thompson Sampling (J=100)                      |     44.19 |       27.5108 |                            13.3325 | 1.02s  |
| Greedy                                                      |     39.00 |       28.0151 |                             9.7636 | 0.08s  |
| UCB-DT (γ=1.00)                                             |     39.38 |       28.0689 |                             9.7290 | 2.60s  |
| TS-UCB (10 samples)                                         |     45.12 |       28.1337 |                             6.0061 | 7.14s  |
| ETC (m=2)                                                   |     39.38 |       28.1505 |                             9.5303 | 0.08s  |
| Bootstrapped Thompson Sampling (J=10)                       |     41.24 |       28.6171 |                            14.3000 | 0.35s  |
| ϵ-Exploring Thompson Sampling                               |     41.08 |       30.8109 |                             9.0357 | 0.14s  |
| TS-UCB (1 samples)                                          |     42.42 |       31.6765 |                             6.1443 | 0.69s  |
| ETC (m=10)                                                  |     43.19 |       31.7349 |                             5.5587 | 0.14s  |
| Forced Exploration                                          |     42.52 |       33.2202 |                             5.6321 | 0.07s  |
| WR-SDA                                                      |     38.17 |       34.3574 |                             7.8687 | 2.60s  |
| UCB1-Tuned                                                  |     39.23 |       36.0362 |                             5.7070 | 0.25s  |
| ReBoot (r=0.25)                                             |     36.27 |       36.8780 |                             8.0532 | 0.19s  |
| Vanilla Residual Bootstrap (init=0)                         |     35.53 |       38.0238 |                             7.8845 | 0.18s  |
| Optimistic Thompson Sampling                                |     37.57 |       38.4989 |                             7.1213 | 0.89s  |
| Multiplier Bootstrap-based Exploration                      |     36.71 |       38.8681 |                             6.9205 | 5.77s  |
| ReBoot (r=0.50)                                             |     34.58 |       39.6438 |                             8.1733 | 0.26s  |
| Thompson Sampling                                           |     35.68 |       40.6934 |                             7.4756 | 0.65s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     35.61 |       40.7462 |                             7.4738 | 0.79s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     35.54 |       40.8342 |                             7.6058 | 0.82s  |
| Garbage In, Reward Out (a=0.10)                             |     34.26 |       42.3026 |                             7.5669 | 1.12s  |
| Perturbed-History Exploration (a=1.1)                       |     34.15 |       42.4480 |                             7.6337 | 0.80s  |
| KL-UCB                                                      |     35.23 |       42.8489 |                             6.2867 | 8.07s  |
| EB-TCI                                                      |     30.68 |       43.1680 |                             8.8295 | 0.36s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     33.15 |       43.2663 |                             8.0491 | 0.92s  |
| Non-Parametric Thompson Sampling                            |     33.66 |       43.8953 |                             7.4578 | 4.34s  |
| Vanilla Residual Bootstrap (init=1)                         |     33.49 |       43.9511 |                             7.4165 | 0.23s  |
| Bounded Dirichlet Sampling                                  |     33.37 |       44.9539 |                             7.9732 | 2.59s  |
| Tsallis-INF                                                 |     33.02 |       45.9683 |                             8.4113 | 1.02s  |
| lil' UCB (δ=0.100)                                          |     32.27 |       46.6215 |                             6.6925 | 0.30s  |
| Kullback-Leibler Maillard Sampling                          |     30.15 |       48.1212 |                             8.2677 | 0.57s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     27.97 |       48.1233 |                            10.0095 | 1.10s  |
| Garbage In, Reward Out (a=0.33)                             |     30.57 |       48.3822 |                             7.9763 | 1.34s  |
| ReBoot (r=0.90)                                             |     29.75 |       48.7258 |                             8.4139 | 0.29s  |
| ReBoot (r=1.00)                                             |     28.27 |       51.2697 |                             8.5685 | 0.26s  |
| Perturbed-History Exploration (a=2.1)                       |     28.34 |       52.5133 |                             8.3130 | 1.18s  |
| Vanilla Residual Bootstrap (init=5)                         |     28.72 |       53.5870 |                             8.3547 | 0.24s  |
| lil' UCB (δ=0.010)                                          |     26.26 |       57.2169 |                             8.1942 | 0.29s  |
| Garbage In, Reward Out (a=1.00)                             |     25.46 |       58.0798 |                             8.9055 | 1.18s  |
| Boltzmann-Gumbel Exploration                                |     25.93 |       58.3994 |                             8.7698 | 0.31s  |
| ReBoot (r=1.50)                                             |     23.11 |       61.4855 |                             9.5988 | 0.25s  |
| lil' UCB (δ=0.001)                                          |     23.15 |       63.1709 |                             9.1364 | 0.26s  |
| ReBoot (r=1.70)                                             |     21.59 |       64.8451 |                            10.1444 | 0.24s  |
| UCB1                                                        |     20.65 |       68.4993 |                            10.1090 | 0.16s  |
| ReBoot (r=2.10)                                             |     19.33 |       70.1924 |                            10.6578 | 0.24s  |
| Gradient Bandit                                             |     19.16 |       75.6775 |                            12.1688 | 0.42s  |
| Gradient Bandit (with baseline)                             |     18.70 |       77.4743 |                            10.5750 | 0.48s  |
| Random                                                      |      9.99 |      102.5290 |                            15.1550 | 0.02s  |
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
| ETC (m=3)                                                   |     16.60 |        4.1698 |                             0.0000 | 0.11s  |
| ETC (m=2)                                                   |     16.60 |        4.1700 |                             0.0000 | 0.08s  |
| Greedy                                                      |     16.60 |        4.1700 |                             0.0100 | 0.07s  |
| ETC (m=5)                                                   |     16.55 |        4.1725 |                             0.0100 | 0.11s  |
| ETC (m=10)                                                  |     16.12 |        4.1940 |                             0.0000 | 0.11s  |
| ϵ-Exploring Thompson Sampling                               |     13.52 |        4.3242 |                             0.1100 | 0.14s  |
| Forced Exploration                                          |     13.38 |        4.3312 |                             0.1000 | 0.06s  |
| UCB-DT (γ=0.90)                                             |     13.15 |        4.3424 |                             0.0100 | 2.50s  |
| UCB-DT (γ=0.95)                                             |     13.15 |        4.3424 |                             0.0100 | 2.46s  |
| UCB-DT (γ=1.00)                                             |     13.07 |        4.3464 |                             0.0200 | 2.47s  |
| UCB-DT (γ=0.75)                                             |     12.93 |        4.3535 |                             0.0100 | 2.47s  |
| TS-UCB (100 samples)                                        |     12.06 |        4.3971 |                             0.2500 | 62.47s |
| Bootstrapped Thompson Sampling (J=10)                       |     11.79 |        4.4106 |                             0.1600 | 0.38s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     11.63 |        4.4187 |                             0.3200 | 8.26s  |
| Bootstrapped Thompson Sampling (J=100)                      |     11.61 |        4.4196 |                             0.2900 | 1.04s  |
| Bootstrapped Thompson Sampling (J=500)                      |     11.61 |        4.4196 |                             0.3100 | 4.29s  |
| TS-UCB (10 samples)                                         |     11.58 |        4.4209 |                             0.4400 | 5.75s  |
| EB-TCI                                                      |     11.50 |        4.4250 |                             0.4400 | 0.32s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     11.48 |        4.4258 |                             0.4000 | 4.55s  |
| WR-SDA                                                      |     11.44 |        4.4278 |                             0.3200 | 1.74s  |
| Vanilla Residual Bootstrap (init=0)                         |     11.36 |        4.4320 |                             0.3500 | 0.17s  |
| ReBoot (r=0.25)                                             |     11.32 |        4.4341 |                             0.3500 | 0.19s  |
| ReBoot (r=0.50)                                             |     11.27 |        4.4363 |                             0.3800 | 0.24s  |
| TS-UCB (1 samples)                                          |     11.26 |        4.4368 |                             0.4600 | 0.61s  |
| Optimistic Thompson Sampling                                |     11.26 |        4.4371 |                             0.4400 | 0.86s  |
| Tsallis-INF                                                 |     11.25 |        4.4377 |                             0.2900 | 1.00s  |
| Non-Parametric Thompson Sampling                            |     11.22 |        4.4391 |                             0.4100 | 4.27s  |
| Vanilla Residual Bootstrap (init=1)                         |     11.21 |        4.4393 |                             0.4200 | 0.21s  |
| Thompson Sampling                                           |     11.21 |        4.4397 |                             0.4300 | 0.60s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     11.20 |        4.4398 |                             0.4400 | 0.79s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     11.20 |        4.4401 |                             0.4400 | 0.85s  |
| Perturbed-History Exploration (a=1.1)                       |     11.20 |        4.4402 |                             0.4300 | 0.85s  |
| Multiplier Bootstrap-based Exploration                      |     11.19 |        4.4406 |                             0.4400 | 5.70s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     11.15 |        4.4426 |                             0.4100 | 0.91s  |
| Garbage In, Reward Out (a=0.10)                             |     11.12 |        4.4441 |                             0.3400 | 1.11s  |
| Garbage In, Reward Out (a=0.33)                             |     11.09 |        4.4455 |                             0.3800 | 1.23s  |
| KL-UCB                                                      |     11.06 |        4.4468 |                             0.3000 | 7.91s  |
| ReBoot (r=0.90)                                             |     11.03 |        4.4485 |                             0.3700 | 0.24s  |
| Perturbed-History Exploration (a=2.1)                       |     10.95 |        4.4524 |                             0.3300 | 0.97s  |
| Kullback-Leibler Maillard Sampling                          |     10.94 |        4.4530 |                             0.3300 | 0.56s  |
| ReBoot (r=1.00)                                             |     10.93 |        4.4537 |                             0.3300 | 0.24s  |
| lil' UCB (δ=0.100)                                          |     10.92 |        4.4539 |                             0.2800 | 0.28s  |
| Vanilla Residual Bootstrap (init=5)                         |     10.92 |        4.4540 |                             0.2800 | 0.22s  |
| Bounded Dirichlet Sampling                                  |     10.91 |        4.4545 |                             0.2900 | 2.45s  |
| UCB1-Tuned                                                  |     10.82 |        4.4591 |                             0.4600 | 0.25s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     10.78 |        4.4612 |                             0.3100 | 0.94s  |
| lil' UCB (δ=0.010)                                          |     10.75 |        4.4625 |                             0.2500 | 0.28s  |
| Boltzmann-Gumbel Exploration                                |     10.73 |        4.4636 |                             0.2600 | 0.34s  |
| Garbage In, Reward Out (a=1.00)                             |     10.71 |        4.4646 |                             0.2600 | 1.15s  |
| lil' UCB (δ=0.001)                                          |     10.59 |        4.4707 |                             0.1700 | 0.26s  |
| ReBoot (r=1.50)                                             |     10.56 |        4.4722 |                             0.2000 | 0.25s  |
| ReBoot (r=1.70)                                             |     10.46 |        4.4771 |                             0.1700 | 0.24s  |
| ReBoot (r=2.10)                                             |     10.34 |        4.4829 |                             0.1400 | 0.25s  |
| UCB1                                                        |     10.26 |        4.4872 |                             0.1300 | 0.14s  |
| Gradient Bandit (with baseline)                             |     10.23 |        4.4885 |                             0.1100 | 0.40s  |
| Gradient Bandit                                             |     10.18 |        4.4912 |                             0.1300 | 0.39s  |
| Random                                                      |      9.98 |        4.5008 |                             0.0400 | 0.02s  |
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
| UCB-DT (γ=0.75)                                             |     55.00 |       22.7051 |                             6.0302 | 2.30s  |
| UCB-DT (γ=0.95)                                             |     54.67 |       22.8374 |                             6.0357 | 2.33s  |
| UCB-DT (γ=0.90)                                             |     54.53 |       22.8662 |                             6.0630 | 2.32s  |
| UCB-DT (γ=1.00)                                             |     53.44 |       22.9767 |                             7.3694 | 2.27s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     56.91 |       23.2902 |                             7.1493 | 17.38s |
| ReBoot (r=0.25)                                             |     52.94 |       24.7856 |                             8.6439 | 0.19s  |
| TS-UCB (100 samples)                                        |     56.19 |       25.1924 |                             4.4774 | 69.07s |
| TS-UCB (10 samples)                                         |     54.99 |       26.7554 |                             4.4802 | 6.86s  |
| ETC (m=5)                                                   |     49.33 |       27.6619 |                             7.2725 | 0.12s  |
| ReBoot (r=0.50)                                             |     51.72 |       29.2332 |                             6.5279 | 0.24s  |
| ETC (m=10)                                                  |     51.27 |       29.6489 |                             8.2708 | 0.13s  |
| TS-UCB (1 samples)                                          |     52.72 |       29.8275 |                             5.0292 | 0.66s  |
| ETC (m=3)                                                   |     45.71 |       30.0356 |                             8.1501 | 0.11s  |
| ETC (m=2)                                                   |     45.37 |       30.2861 |                             8.1393 | 0.09s  |
| Bootstrapped Thompson Sampling (J=10)                       |     50.33 |       31.3906 |                             6.7436 | 0.36s  |
| Forced Exploration                                          |     49.27 |       31.6725 |                             8.8808 | 0.07s  |
| Multiplier Bootstrap-based Exploration                      |     49.58 |       32.6088 |                             6.2663 | 5.84s  |
| ϵ-Exploring Thompson Sampling                               |     44.70 |       33.6912 |                            12.4300 | 0.15s  |
| UCB1-Tuned                                                  |     48.78 |       34.1720 |                             5.7265 | 0.26s  |
| Bootstrapped Thompson Sampling (J=100)                      |     47.62 |       34.9846 |                             6.5196 | 1.04s  |
| Garbage In, Reward Out (a=0.10)                             |     47.11 |       35.3159 |                             6.5716 | 0.89s  |
| Bootstrapped Thompson Sampling (J=500)                      |     47.24 |       35.4846 |                             6.5623 | 4.26s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     47.25 |       35.5259 |                             6.5145 | 8.23s  |
| Vanilla Residual Bootstrap (init=1)                         |     47.27 |       35.6021 |                             6.3890 | 0.21s  |
| Optimistic Thompson Sampling                                |     47.54 |       36.0169 |                             6.2395 | 0.96s  |
| Vanilla Residual Bootstrap (init=0)                         |     39.97 |       36.7298 |                            15.5781 | 0.17s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     45.53 |       38.0235 |                             6.6411 | 0.89s  |
| Thompson Sampling                                           |     45.50 |       38.0338 |                             6.6413 | 0.72s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     45.41 |       38.1336 |                             6.6271 | 0.98s  |
| KL-UCB                                                      |     45.14 |       38.3011 |                             5.9485 | 7.59s  |
| Non-Parametric Thompson Sampling                            |     44.28 |       39.6896 |                             6.8661 | 4.32s  |
| Greedy                                                      |     37.36 |       39.9645 |                            20.3130 | 0.08s  |
| Bounded Dirichlet Sampling                                  |     44.03 |       40.2371 |                             6.7909 | 2.47s  |
| WR-SDA                                                      |     37.82 |       40.8505 |                            18.3470 | 2.81s  |
| ReBoot (r=0.90)                                             |     42.91 |       41.1146 |                             7.3175 | 0.23s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     41.92 |       41.3247 |                             7.3104 | 1.03s  |
| Kullback-Leibler Maillard Sampling                          |     41.32 |       41.7427 |                             7.4157 | 0.51s  |
| Perturbed-History Exploration (a=1.1)                       |     41.26 |       43.0633 |                             7.6161 | 0.86s  |
| ReBoot (r=1.00)                                             |     41.01 |       43.7015 |                             7.7312 | 0.24s  |
| Garbage In, Reward Out (a=0.33)                             |     39.20 |       45.5334 |                             7.9039 | 1.10s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     33.92 |       48.8980 |                             9.5939 | 1.00s  |
| lil' UCB (δ=0.100)                                          |     36.67 |       49.0887 |                             7.5057 | 0.28s  |
| ReBoot (r=1.50)                                             |     33.52 |       54.1198 |                            10.7651 | 0.26s  |
| Perturbed-History Exploration (a=2.1)                       |     33.06 |       54.2431 |                             9.6641 | 1.02s  |
| Tsallis-INF                                                 |     32.65 |       55.1568 |                            11.1605 | 1.06s  |
| Vanilla Residual Bootstrap (init=5)                         |     31.59 |       56.9543 |                            10.2200 | 0.21s  |
| ReBoot (r=1.70)                                             |     31.29 |       57.3346 |                            11.9602 | 0.26s  |
| EB-TCI                                                      |     24.85 |       58.9761 |                            22.9968 | 0.29s  |
| Garbage In, Reward Out (a=1.00)                             |     29.72 |       58.9769 |                            11.3139 | 1.20s  |
| Boltzmann-Gumbel Exploration                                |     30.21 |       59.0762 |                            11.4529 | 0.33s  |
| lil' UCB (δ=0.010)                                          |     29.49 |       59.3792 |                            11.2005 | 0.29s  |
| ReBoot (r=2.10)                                             |     27.81 |       62.5734 |                            14.2399 | 0.26s  |
| lil' UCB (δ=0.001)                                          |     25.59 |       65.3146 |                            14.4606 | 0.26s  |
| UCB1                                                        |     22.44 |       70.4627 |                            16.8609 | 0.16s  |
| Gradient Bandit                                             |     20.43 |       75.0125 |                            17.3070 | 0.40s  |
| Gradient Bandit (with baseline)                             |     20.06 |       75.7085 |                            17.5892 | 0.46s  |
| Random                                                      |      9.99 |       94.2791 |                            25.9206 | 0.02s  |
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
| TS-UCB (100 samples)                                        |     58.71 |        7.4481 |                             2.1886 | 63.31s |
| TS-UCB (10 samples)                                         |     57.79 |        7.8999 |                             1.9148 | 6.36s  |
| TS-UCB (1 samples)                                          |     57.53 |        8.3487 |                             1.7839 | 0.62s  |
| UCB-DT (γ=1.00)                                             |     55.22 |        8.6731 |                             1.5458 | 2.48s  |
| UCB-DT (γ=0.90)                                             |     55.32 |        8.7670 |                             1.5465 | 2.46s  |
| UCB-DT (γ=0.95)                                             |     55.25 |        8.7822 |                             1.5484 | 2.47s  |
| Greedy                                                      |     53.46 |        8.8426 |                             1.5877 | 0.08s  |
| UCB-DT (γ=0.75)                                             |     55.50 |        8.8734 |                             1.5938 | 2.43s  |
| ETC (m=2)                                                   |     53.32 |        8.9559 |                             1.5806 | 0.08s  |
| ETC (m=3)                                                   |     53.18 |        9.1361 |                             1.5916 | 0.11s  |
| Optimistic Thompson Sampling                                |     55.57 |        9.3600 |                             3.3451 | 0.81s  |
| ETC (m=5)                                                   |     52.63 |        9.7543 |                             1.6533 | 0.11s  |
| WR-SDA                                                      |     52.20 |       10.4022 |                             2.8202 | 0.93s  |
| ϵ-Exploring Thompson Sampling                               |     44.32 |       11.1621 |                             4.2373 | 0.14s  |
| KL-UCB                                                      |     51.72 |       11.7591 |                             3.6041 | 6.34s  |
| ETC (m=10)                                                  |     50.72 |       12.5349 |                             2.2955 | 0.12s  |
| Thompson Sampling                                           |     48.36 |       12.6305 |                             2.8003 | 0.61s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     36.88 |       12.6832 |                             4.2582 | 13.58s |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     48.28 |       12.7174 |                             2.8361 | 0.83s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     46.43 |       13.2106 |                             2.8578 | 0.90s  |
| Non-Parametric Thompson Sampling                            |     47.42 |       13.7743 |                             4.3390 | 4.33s  |
| Forced Exploration                                          |     48.03 |       14.2409 |                             2.5482 | 0.07s  |
| Bounded Dirichlet Sampling                                  |     45.50 |       14.7444 |                             4.6974 | 2.18s  |
| Kullback-Leibler Maillard Sampling                          |     43.49 |       15.3254 |                             5.1663 | 0.46s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     27.59 |       18.2837 |                             5.3096 | 0.92s  |
| EB-TCI                                                      |     35.83 |       20.0130 |                             5.2114 | 0.28s  |
| ReBoot (r=0.25)                                             |     34.86 |       20.2055 |                             3.2261 | 0.27s  |
| Vanilla Residual Bootstrap (init=0)                         |     33.47 |       21.7507 |                             3.2564 | 0.17s  |
| Multiplier Bootstrap-based Exploration                      |     28.54 |       22.5267 |                             3.5576 | 6.09s  |
| ReBoot (r=0.50)                                             |     30.85 |       22.7688 |                             3.8323 | 0.31s  |
| UCB1-Tuned                                                  |     25.26 |       23.1257 |                             3.4924 | 0.26s  |
| Vanilla Residual Bootstrap (init=1)                         |     31.25 |       23.4112 |                             3.3759 | 0.20s  |
| Tsallis-INF                                                 |     26.49 |       23.5590 |                             4.3226 | 0.95s  |
| Vanilla Residual Bootstrap (init=5)                         |     30.55 |       24.1464 |                             3.5251 | 0.21s  |
| Garbage In, Reward Out (a=0.10)                             |     26.92 |       24.2118 |                             3.9104 | 0.87s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     17.38 |       25.0755 |                             9.0207 | 0.92s  |
| Perturbed-History Exploration (a=1.1)                       |     24.23 |       25.1162 |                             4.2813 | 0.90s  |
| Bootstrapped Thompson Sampling (J=100)                      |     17.66 |       28.2224 |                            16.7306 | 1.02s  |
| Bootstrapped Thompson Sampling (J=500)                      |     17.69 |       28.2561 |                            16.8472 | 4.21s  |
| Garbage In, Reward Out (a=0.33)                             |     21.30 |       28.3366 |                             4.7374 | 1.16s  |
| ReBoot (r=0.90)                                             |     24.03 |       28.5844 |                             5.1178 | 0.29s  |
| lil' UCB (δ=0.100)                                          |     19.28 |       28.8759 |                             4.7214 | 0.27s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     17.36 |       28.9676 |                            17.3819 | 8.38s  |
| Bootstrapped Thompson Sampling (J=10)                       |     16.96 |       30.0178 |                            18.1195 | 0.35s  |
| ReBoot (r=1.00)                                             |     22.47 |       30.1561 |                             5.4255 | 0.26s  |
| Perturbed-History Exploration (a=2.1)                       |     18.80 |       30.7373 |                             5.2197 | 1.02s  |
| lil' UCB (δ=0.010)                                          |     16.77 |       32.6000 |                             5.5344 | 0.29s  |
| Garbage In, Reward Out (a=1.00)                             |     17.33 |       32.8421 |                             5.6612 | 1.18s  |
| Boltzmann-Gumbel Exploration                                |     17.50 |       33.1221 |                             5.5971 | 0.32s  |
| lil' UCB (δ=0.001)                                          |     15.55 |       34.6643 |                             5.9113 | 0.25s  |
| ReBoot (r=1.50)                                             |     18.16 |       35.6912 |                             6.5617 | 0.28s  |
| UCB1                                                        |     14.58 |       36.5304 |                             6.3337 | 0.16s  |
| ReBoot (r=1.70)                                             |     17.19 |       37.2245 |                             6.9281 | 0.25s  |
| ReBoot (r=2.10)                                             |     15.84 |       39.6794 |                             7.4686 | 0.24s  |
| Gradient Bandit                                             |     13.75 |       39.9529 |                             8.1144 | 0.37s  |
| Gradient Bandit (with baseline)                             |     13.20 |       41.3526 |                             7.4311 | 0.42s  |
| Random                                                      |      9.97 |       49.8281 |                             9.9126 | 0.02s  |
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

- TS-UCB seems to be the best algorithm for me overall; when extending the time
  horizon beyond 500 steps (not shown above), it almost always comes out on top,
  but even on short horizons it does relatively well. It is also very flexible
  with regard to the number of samples, and thus the amount of time it takes to
  run it. It is also easier to tune than UCB-DT -- you just use as many samples
  as you can afford.

- ϵ-Exploring Thompson Sampling seems to match or exceed Thompson Sampling
  while being computationally much lighter.
