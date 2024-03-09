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
- [Forced Exploration (Linear)](https://arxiv.org/abs/2312.07285)
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
| Bootstrapped Thompson Sampling (J=500)                      |     81.55 |       12.3112 |                             1.6871 | 4.26s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     81.44 |       12.6758 |                             1.7121 | 8.88s  |
| Vanilla Residual Bootstrap (init=1)                         |     73.31 |       16.9644 |                             3.0372 | 0.23s  |
| TS-UCB (100 samples)                                        |     72.43 |       17.4061 |                             3.2706 | 64.01s |
| TS-UCB (10 samples)                                         |     72.88 |       17.8546 |                             3.5976 | 6.66s  |
| UCB-DT (γ=1.00)                                             |     70.73 |       18.2706 |                             2.5295 | 2.65s  |
| UCB-DT (γ=0.90)                                             |     73.02 |       18.3178 |                             2.4600 | 2.50s  |
| UCB-DT (γ=0.95)                                             |     72.98 |       18.3323 |                             2.4505 | 2.82s  |
| UCB-DT (γ=0.75)                                             |     72.98 |       18.3605 |                             2.4852 | 2.54s  |
| Vanilla Residual Bootstrap (init=5)                         |     74.14 |       18.3977 |                             4.6928 | 0.25s  |
| Vanilla Residual Bootstrap (init=0)                         |     70.53 |       18.4180 |                             2.5125 | 0.17s  |
| ReBoot (r=0.50)                                             |     69.90 |       18.5399 |                             2.5231 | 0.25s  |
| ReBoot (r=0.25)                                             |     69.10 |       18.8862 |                             2.4942 | 0.20s  |
| ReBoot (r=0.90)                                             |     70.89 |       19.0026 |                             2.8707 | 0.25s  |
| ReBoot (r=1.00)                                             |     70.75 |       19.6511 |                             2.9337 | 0.23s  |
| Greedy                                                      |     67.48 |       19.7483 |                             2.4973 | 0.07s  |
| TS-UCB (1 samples)                                          |     72.28 |       19.9767 |                             5.3785 | 0.72s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     63.36 |       21.1298 |                             6.2710 | 25.80s |
| WR-SDA                                                      |     67.66 |       23.8199 |                             5.0460 | 1.80s  |
| Optimistic Thompson Sampling                                |     69.69 |       25.4924 |                             7.1978 | 0.88s  |
| Multiplier Bootstrap-based Exploration                      |     67.82 |       26.0614 |                             3.6393 | 6.14s  |
| ReBoot (r=1.50)                                             |     70.37 |       26.4939 |                             3.5531 | 0.25s  |
| ϵ-Exploring Thompson Sampling                               |     64.31 |       27.5471 |                             8.9868 | 0.14s  |
| Thompson Sampling                                           |     67.00 |       28.9445 |                             7.1632 | 0.70s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     66.88 |       29.0225 |                             7.0900 | 0.85s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     66.49 |       29.3398 |                             6.9895 | 0.94s  |
| KL-UCB                                                      |     67.56 |       29.6893 |                             7.4957 | 7.89s  |
| ReBoot (r=1.70)                                             |     68.47 |       31.4177 |                             3.6511 | 0.25s  |
| UCB1-Tuned                                                  |     62.81 |       31.7769 |                             3.6345 | 0.24s  |
| Bootstrapped Thompson Sampling (J=100)                      |     51.85 |       32.8060 |                            13.5494 | 1.03s  |
| Non-Parametric Thompson Sampling                            |     64.59 |       33.8504 |                             7.0679 | 4.71s  |
| Bounded Dirichlet Sampling                                  |     64.70 |       34.2376 |                             7.1518 | 2.31s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     58.20 |       34.9791 |                             6.9401 | 0.96s  |
| Kullback-Leibler Maillard Sampling                          |     60.53 |       37.5467 |                             8.4138 | 0.53s  |
| Perturbed-History Exploration (a=1.1)                       |     57.78 |       37.8970 |                             5.6488 | 0.80s  |
| Bootstrapped Thompson Sampling (J=10)                       |     45.02 |       39.1305 |                            19.4246 | 0.37s  |
| ReBoot (r=2.10)                                             |     63.19 |       42.4910 |                             4.3076 | 0.25s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     44.92 |       44.1840 |                            10.6738 | 1.00s  |
| Garbage In, Reward Out (a=0.10)                             |     57.08 |       44.4496 |                             4.8697 | 0.89s  |
| lil' UCB (δ=0.100)                                          |     52.87 |       44.9486 |                             5.5879 | 0.31s  |
| Tsallis-INF                                                 |     55.26 |       46.5441 |                             5.8549 | 1.15s  |
| Garbage In, Reward Out (a=0.33)                             |     51.88 |       51.5502 |                             5.3784 | 1.15s  |
| EB-TCI                                                      |     42.95 |       56.0202 |                            16.1098 | 0.32s  |
| Perturbed-History Exploration (a=2.1)                       |     48.19 |       56.7164 |                             6.0494 | 1.19s  |
| lil' UCB (δ=0.010)                                          |     44.60 |       62.4201 |                             6.5913 | 0.30s  |
| Garbage In, Reward Out (a=1.00)                             |     43.64 |       66.8026 |                             7.0771 | 1.08s  |
| Boltzmann-Gumbel Exploration                                |     44.52 |       69.1820 |                             6.7076 | 0.35s  |
| lil' UCB (δ=0.001)                                          |     39.59 |       74.2236 |                             8.0274 | 0.26s  |
| UCB1                                                        |     34.84 |       87.3965 |                            10.1205 | 0.16s  |
| Gradient Bandit                                             |     30.56 |      111.1047 |                            17.4381 | 0.37s  |
| Gradient Bandit (with baseline)                             |     31.78 |      114.0673 |                            11.6366 | 0.45s  |
| Forced Exploration                                          |     39.67 |      120.7367 |                            16.8185 | 0.06s  |
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
| Bootstrapped Thompson Sampling (J=500)                      |     60.67 |       20.0135 |                            10.8059 | 4.54s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     60.81 |       20.0744 |                            10.7234 | 9.02s  |
| Vanilla Residual Bootstrap (init=1)                         |     45.94 |       24.6010 |                             6.5389 | 0.23s  |
| UCB-DT (γ=0.90)                                             |     44.11 |       25.7379 |                             7.1522 | 2.75s  |
| UCB-DT (γ=0.95)                                             |     44.07 |       25.7444 |                             7.1627 | 2.78s  |
| UCB-DT (γ=0.75)                                             |     44.20 |       25.7518 |                             7.1508 | 2.68s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     44.83 |       26.7704 |                             8.7872 | 12.20s |
| TS-UCB (100 samples)                                        |     44.83 |       27.4483 |                             6.6267 | 68.43s |
| Bootstrapped Thompson Sampling (J=100)                      |     44.19 |       27.5108 |                            13.3325 | 1.10s  |
| Vanilla Residual Bootstrap (init=0)                         |     39.98 |       27.7827 |                             9.2352 | 0.17s  |
| ReBoot (r=1.00)                                             |     41.18 |       27.8871 |                             8.3985 | 0.25s  |
| ReBoot (r=0.25)                                             |     39.43 |       27.9227 |                             9.4988 | 0.20s  |
| Greedy                                                      |     39.00 |       28.0151 |                             9.7636 | 0.09s  |
| ReBoot (r=0.90)                                             |     40.63 |       28.0172 |                             8.8135 | 0.30s  |
| UCB-DT (γ=1.00)                                             |     39.38 |       28.0689 |                             9.7290 | 2.67s  |
| ReBoot (r=0.50)                                             |     39.52 |       28.0805 |                             9.6491 | 0.30s  |
| TS-UCB (10 samples)                                         |     45.12 |       28.1337 |                             6.0061 | 7.11s  |
| Bootstrapped Thompson Sampling (J=10)                       |     41.24 |       28.6171 |                            14.3000 | 0.35s  |
| Vanilla Residual Bootstrap (init=5)                         |     43.54 |       30.2281 |                             6.9636 | 0.24s  |
| ϵ-Exploring Thompson Sampling                               |     41.08 |       30.8109 |                             9.0357 | 0.14s  |
| Multiplier Bootstrap-based Exploration                      |     42.47 |       30.9818 |                             6.6402 | 6.16s  |
| TS-UCB (1 samples)                                          |     42.42 |       31.6765 |                             6.1443 | 0.72s  |
| ReBoot (r=1.50)                                             |     42.27 |       31.7111 |                             6.1746 | 0.26s  |
| WR-SDA                                                      |     38.17 |       34.3574 |                             7.8687 | 2.72s  |
| ReBoot (r=1.70)                                             |     39.81 |       35.3730 |                             6.1512 | 0.27s  |
| UCB1-Tuned                                                  |     39.23 |       36.0362 |                             5.7070 | 0.27s  |
| Optimistic Thompson Sampling                                |     37.57 |       38.4989 |                             7.1213 | 0.94s  |
| Thompson Sampling                                           |     35.68 |       40.6934 |                             7.4756 | 0.70s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     35.61 |       40.7462 |                             7.4738 | 0.93s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     35.54 |       40.8342 |                             7.6058 | 1.02s  |
| ReBoot (r=2.10)                                             |     36.02 |       41.5702 |                             6.5876 | 0.26s  |
| Perturbed-History Exploration (a=1.1)                       |     34.15 |       42.4480 |                             7.6337 | 0.87s  |
| KL-UCB                                                      |     35.22 |       42.8549 |                             6.2878 | 8.30s  |
| EB-TCI                                                      |     30.68 |       43.1680 |                             8.8295 | 0.34s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     33.15 |       43.2663 |                             8.0491 | 1.05s  |
| Non-Parametric Thompson Sampling                            |     33.66 |       43.8953 |                             7.4578 | 4.71s  |
| Bounded Dirichlet Sampling                                  |     33.37 |       44.9539 |                             7.9732 | 2.75s  |
| Garbage In, Reward Out (a=0.10)                             |     32.82 |       44.9909 |                             7.5012 | 1.25s  |
| Tsallis-INF                                                 |     33.02 |       45.9683 |                             8.4113 | 1.13s  |
| lil' UCB (δ=0.100)                                          |     32.27 |       46.6215 |                             6.6925 | 0.29s  |
| Kullback-Leibler Maillard Sampling                          |     30.15 |       48.1212 |                             8.2677 | 0.52s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     27.97 |       48.1233 |                            10.0095 | 1.04s  |
| Garbage In, Reward Out (a=0.33)                             |     30.19 |       49.2192 |                             8.0236 | 1.24s  |
| Perturbed-History Exploration (a=2.1)                       |     28.34 |       52.5133 |                             8.3130 | 1.14s  |
| lil' UCB (δ=0.010)                                          |     26.26 |       57.2169 |                             8.1942 | 0.30s  |
| Garbage In, Reward Out (a=1.00)                             |     25.47 |       58.0660 |                             8.8999 | 1.26s  |
| Boltzmann-Gumbel Exploration                                |     25.93 |       58.3994 |                             8.7698 | 0.41s  |
| lil' UCB (δ=0.001)                                          |     23.15 |       63.1709 |                             9.1364 | 0.27s  |
| Forced Exploration                                          |     27.38 |       65.0601 |                             9.4003 | 0.05s  |
| UCB1                                                        |     20.65 |       68.4993 |                            10.1090 | 0.17s  |
| Gradient Bandit                                             |     19.16 |       75.6775 |                            12.1688 | 0.40s  |
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
| Greedy                                                      |     16.60 |        4.1700 |                             0.0100 | 0.07s  |
| ReBoot (r=0.25)                                             |     14.23 |        4.2883 |                             0.0200 | 0.18s  |
| Vanilla Residual Bootstrap (init=0)                         |     13.97 |        4.3017 |                             0.0600 | 0.17s  |
| ϵ-Exploring Thompson Sampling                               |     13.52 |        4.3242 |                             0.1100 | 0.14s  |
| UCB-DT (γ=0.90)                                             |     13.15 |        4.3424 |                             0.0100 | 2.59s  |
| UCB-DT (γ=0.95)                                             |     13.15 |        4.3424 |                             0.0100 | 2.54s  |
| UCB-DT (γ=1.00)                                             |     13.07 |        4.3464 |                             0.0200 | 2.65s  |
| ReBoot (r=0.50)                                             |     13.07 |        4.3467 |                             0.0200 | 0.22s  |
| Forced Exploration                                          |     13.05 |        4.3476 |                             0.1900 | 0.06s  |
| UCB-DT (γ=0.75)                                             |     12.93 |        4.3535 |                             0.0100 | 2.60s  |
| Vanilla Residual Bootstrap (init=1)                         |     12.84 |        4.3582 |                             0.1700 | 0.22s  |
| ReBoot (r=0.90)                                             |     12.62 |        4.3688 |                             0.1300 | 0.24s  |
| ReBoot (r=1.00)                                             |     12.46 |        4.3769 |                             0.1800 | 0.22s  |
| TS-UCB (100 samples)                                        |     12.06 |        4.3971 |                             0.2500 | 67.36s |
| Bootstrapped Thompson Sampling (J=10)                       |     11.79 |        4.4106 |                             0.1600 | 0.46s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     11.63 |        4.4187 |                             0.3200 | 8.43s  |
| ReBoot (r=1.50)                                             |     11.62 |        4.4190 |                             0.3800 | 0.24s  |
| Bootstrapped Thompson Sampling (J=100)                      |     11.61 |        4.4196 |                             0.2900 | 1.21s  |
| Bootstrapped Thompson Sampling (J=500)                      |     11.61 |        4.4196 |                             0.3100 | 4.36s  |
| TS-UCB (10 samples)                                         |     11.58 |        4.4209 |                             0.4400 | 7.03s  |
| Vanilla Residual Bootstrap (init=5)                         |     11.51 |        4.4247 |                             0.4400 | 0.22s  |
| EB-TCI                                                      |     11.50 |        4.4250 |                             0.4400 | 0.33s  |
| Multiplier Bootstrap-based Exploration                      |     11.49 |        4.4257 |                             0.2500 | 5.78s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     11.48 |        4.4258 |                             0.4000 | 4.85s  |
| WR-SDA                                                      |     11.44 |        4.4278 |                             0.3200 | 1.79s  |
| ReBoot (r=1.70)                                             |     11.34 |        4.4332 |                             0.4200 | 0.22s  |
| TS-UCB (1 samples)                                          |     11.26 |        4.4368 |                             0.4600 | 0.64s  |
| Optimistic Thompson Sampling                                |     11.26 |        4.4371 |                             0.4400 | 0.91s  |
| Tsallis-INF                                                 |     11.25 |        4.4377 |                             0.2900 | 1.15s  |
| ReBoot (r=2.10)                                             |     11.24 |        4.4380 |                             0.4200 | 0.22s  |
| Non-Parametric Thompson Sampling                            |     11.22 |        4.4391 |                             0.4100 | 4.60s  |
| Thompson Sampling                                           |     11.21 |        4.4397 |                             0.4300 | 0.59s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     11.20 |        4.4398 |                             0.4400 | 0.85s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     11.20 |        4.4401 |                             0.4400 | 1.00s  |
| Perturbed-History Exploration (a=1.1)                       |     11.20 |        4.4402 |                             0.4300 | 0.94s  |
| Garbage In, Reward Out (a=0.10)                             |     11.19 |        4.4404 |                             0.4200 | 1.07s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     11.15 |        4.4426 |                             0.4100 | 0.89s  |
| Garbage In, Reward Out (a=0.33)                             |     11.08 |        4.4462 |                             0.3700 | 1.10s  |
| KL-UCB                                                      |     11.06 |        4.4468 |                             0.3000 | 8.23s  |
| Perturbed-History Exploration (a=2.1)                       |     10.95 |        4.4524 |                             0.3300 | 1.03s  |
| Kullback-Leibler Maillard Sampling                          |     10.94 |        4.4530 |                             0.3300 | 0.60s  |
| lil' UCB (δ=0.100)                                          |     10.92 |        4.4539 |                             0.2800 | 0.32s  |
| Bounded Dirichlet Sampling                                  |     10.91 |        4.4545 |                             0.2900 | 2.56s  |
| UCB1-Tuned                                                  |     10.82 |        4.4591 |                             0.4600 | 0.26s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     10.78 |        4.4612 |                             0.3100 | 0.96s  |
| lil' UCB (δ=0.010)                                          |     10.75 |        4.4625 |                             0.2500 | 0.34s  |
| Boltzmann-Gumbel Exploration                                |     10.73 |        4.4636 |                             0.2600 | 0.35s  |
| Garbage In, Reward Out (a=1.00)                             |     10.72 |        4.4642 |                             0.2600 | 1.17s  |
| lil' UCB (δ=0.001)                                          |     10.59 |        4.4707 |                             0.1700 | 0.28s  |
| UCB1                                                        |     10.26 |        4.4872 |                             0.1300 | 0.14s  |
| Gradient Bandit (with baseline)                             |     10.23 |        4.4885 |                             0.1100 | 0.44s  |
| Gradient Bandit                                             |     10.18 |        4.4912 |                             0.1300 | 0.36s  |
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
| Vanilla Residual Bootstrap (init=1)                         |     56.71 |       22.3221 |                             4.6914 | 0.20s  |
| ReBoot (r=1.00)                                             |     55.00 |       22.7015 |                             5.7422 | 0.24s  |
| UCB-DT (γ=0.75)                                             |     55.00 |       22.7051 |                             6.0302 | 2.30s  |
| UCB-DT (γ=0.95)                                             |     54.67 |       22.8374 |                             6.0357 | 2.33s  |
| UCB-DT (γ=0.90)                                             |     54.53 |       22.8662 |                             6.0630 | 2.33s  |
| UCB-DT (γ=1.00)                                             |     53.44 |       22.9767 |                             7.3694 | 2.28s  |
| ReBoot (r=0.90)                                             |     53.45 |       23.1910 |                             6.5038 | 0.25s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     56.91 |       23.2902 |                             7.1493 | 17.30s |
| TS-UCB (100 samples)                                        |     56.19 |       25.1924 |                             4.4774 | 66.27s |
| ReBoot (r=1.50)                                             |     55.33 |       25.5983 |                             4.8770 | 0.27s  |
| Multiplier Bootstrap-based Exploration                      |     54.92 |       25.7531 |                             5.7460 | 5.97s  |
| TS-UCB (10 samples)                                         |     54.99 |       26.7554 |                             4.4802 | 5.95s  |
| ReBoot (r=1.70)                                             |     54.15 |       27.8226 |                             5.3817 | 0.28s  |
| TS-UCB (1 samples)                                          |     52.72 |       29.8275 |                             5.0292 | 0.64s  |
| ReBoot (r=0.50)                                             |     44.19 |       30.2711 |                            12.3522 | 0.26s  |
| Bootstrapped Thompson Sampling (J=10)                       |     50.33 |       31.3906 |                             6.7436 | 0.37s  |
| ReBoot (r=2.10)                                             |     51.74 |       32.4011 |                             6.6040 | 0.29s  |
| ϵ-Exploring Thompson Sampling                               |     44.70 |       33.6912 |                            12.4300 | 0.14s  |
| UCB1-Tuned                                                  |     48.78 |       34.1720 |                             5.7265 | 0.25s  |
| Bootstrapped Thompson Sampling (J=100)                      |     47.62 |       34.9846 |                             6.5196 | 1.05s  |
| Bootstrapped Thompson Sampling (J=500)                      |     47.24 |       35.4846 |                             6.5623 | 4.43s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     47.25 |       35.5259 |                             6.5145 | 8.45s  |
| Vanilla Residual Bootstrap (init=5)                         |     48.61 |       35.5557 |                             5.6832 | 0.20s  |
| Optimistic Thompson Sampling                                |     47.54 |       36.0169 |                             6.2395 | 0.99s  |
| Garbage In, Reward Out (a=0.10)                             |     46.27 |       36.5880 |                             6.6192 | 0.81s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     45.53 |       38.0235 |                             6.6411 | 1.00s  |
| Thompson Sampling                                           |     45.50 |       38.0338 |                             6.6413 | 0.64s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     45.41 |       38.1336 |                             6.6271 | 1.01s  |
| KL-UCB                                                      |     45.13 |       38.3085 |                             5.9510 | 7.69s  |
| Vanilla Residual Bootstrap (init=0)                         |     38.13 |       39.3544 |                            19.2688 | 0.16s  |
| ReBoot (r=0.25)                                             |     37.92 |       39.3729 |                            19.8975 | 0.19s  |
| Non-Parametric Thompson Sampling                            |     44.28 |       39.6896 |                             6.8661 | 4.30s  |
| Greedy                                                      |     37.36 |       39.9645 |                            20.3130 | 0.09s  |
| Bounded Dirichlet Sampling                                  |     44.03 |       40.2371 |                             6.7909 | 2.57s  |
| WR-SDA                                                      |     37.82 |       40.8505 |                            18.3470 | 2.89s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     41.92 |       41.3247 |                             7.3104 | 1.00s  |
| Kullback-Leibler Maillard Sampling                          |     41.32 |       41.7427 |                             7.4157 | 0.48s  |
| Perturbed-History Exploration (a=1.1)                       |     41.26 |       43.0633 |                             7.6161 | 0.86s  |
| Garbage In, Reward Out (a=0.33)                             |     38.72 |       46.2679 |                             7.9517 | 1.10s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     33.92 |       48.8980 |                             9.5939 | 1.04s  |
| lil' UCB (δ=0.100)                                          |     36.67 |       49.0887 |                             7.5057 | 0.30s  |
| Perturbed-History Exploration (a=2.1)                       |     33.06 |       54.2431 |                             9.6641 | 1.17s  |
| Tsallis-INF                                                 |     32.65 |       55.1568 |                            11.1605 | 1.06s  |
| Forced Exploration                                          |     33.93 |       58.8258 |                            16.0080 | 0.06s  |
| EB-TCI                                                      |     24.85 |       58.9761 |                            22.9968 | 0.29s  |
| Garbage In, Reward Out (a=1.00)                             |     29.74 |       58.9955 |                            11.3563 | 1.13s  |
| Boltzmann-Gumbel Exploration                                |     30.21 |       59.0762 |                            11.4529 | 0.32s  |
| lil' UCB (δ=0.010)                                          |     29.49 |       59.3792 |                            11.2005 | 0.31s  |
| lil' UCB (δ=0.001)                                          |     25.59 |       65.3146 |                            14.4606 | 0.28s  |
| UCB1                                                        |     22.44 |       70.4627 |                            16.8609 | 0.15s  |
| Gradient Bandit                                             |     20.43 |       75.0125 |                            17.3070 | 0.37s  |
| Gradient Bandit (with baseline)                             |     20.06 |       75.7085 |                            17.5892 | 0.42s  |
| Random                                                      |      9.99 |       94.2791 |                            25.9206 | 0.03s  |
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
| TS-UCB (100 samples)                                        |     58.71 |        7.4481 |                             2.1886 | 58.03s |
| TS-UCB (10 samples)                                         |     57.79 |        7.8999 |                             1.9148 | 5.62s  |
| TS-UCB (1 samples)                                          |     57.53 |        8.3487 |                             1.7839 | 0.61s  |
| Vanilla Residual Bootstrap (init=5)                         |     54.98 |        8.6023 |                             1.5733 | 0.20s  |
| Vanilla Residual Bootstrap (init=1)                         |     54.29 |        8.6730 |                             1.5690 | 0.20s  |
| UCB-DT (γ=1.00)                                             |     55.22 |        8.6731 |                             1.5458 | 2.51s  |
| ReBoot (r=0.50)                                             |     53.85 |        8.7544 |                             1.5784 | 0.22s  |
| Vanilla Residual Bootstrap (init=0)                         |     53.82 |        8.7563 |                             1.5834 | 0.17s  |
| UCB-DT (γ=0.90)                                             |     55.32 |        8.7670 |                             1.5465 | 2.47s  |
| UCB-DT (γ=0.95)                                             |     55.25 |        8.7822 |                             1.5484 | 2.47s  |
| ReBoot (r=0.25)                                             |     53.65 |        8.7916 |                             1.5797 | 0.18s  |
| ReBoot (r=0.90)                                             |     54.52 |        8.8017 |                             1.5919 | 0.22s  |
| Greedy                                                      |     53.46 |        8.8426 |                             1.5877 | 0.08s  |
| UCB-DT (γ=0.75)                                             |     55.50 |        8.8734 |                             1.5938 | 2.44s  |
| ReBoot (r=1.00)                                             |     54.58 |        8.9873 |                             1.6223 | 0.23s  |
| Optimistic Thompson Sampling                                |     55.57 |        9.3600 |                             3.3451 | 0.79s  |
| WR-SDA                                                      |     52.20 |       10.4022 |                             2.8202 | 0.89s  |
| ϵ-Exploring Thompson Sampling                               |     44.32 |       11.1621 |                             4.2373 | 0.13s  |
| KL-UCB                                                      |     51.72 |       11.7599 |                             3.6028 | 6.19s  |
| Thompson Sampling                                           |     48.36 |       12.6305 |                             2.8003 | 0.61s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     36.88 |       12.6832 |                             4.2582 | 13.46s |
| ReBoot (r=1.50)                                             |     50.83 |       12.6931 |                             2.2936 | 0.23s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     48.28 |       12.7174 |                             2.8361 | 0.86s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     46.43 |       13.2106 |                             2.8578 | 0.89s  |
| Non-Parametric Thompson Sampling                            |     47.42 |       13.7743 |                             4.3390 | 4.22s  |
| Bounded Dirichlet Sampling                                  |     45.50 |       14.7444 |                             4.6974 | 2.18s  |
| ReBoot (r=1.70)                                             |     48.26 |       14.9293 |                             2.5980 | 0.23s  |
| Kullback-Leibler Maillard Sampling                          |     43.49 |       15.3254 |                             5.1663 | 0.46s  |
| Multiplier Bootstrap-based Exploration                      |     37.02 |       17.2756 |                             2.6160 | 5.65s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     27.59 |       18.2837 |                             5.3096 | 0.91s  |
| ReBoot (r=2.10)                                             |     42.67 |       19.2490 |                             3.3197 | 0.22s  |
| EB-TCI                                                      |     35.83 |       20.0130 |                             5.2114 | 0.30s  |
| UCB1-Tuned                                                  |     25.26 |       23.1257 |                             3.4924 | 0.26s  |
| Tsallis-INF                                                 |     26.49 |       23.5590 |                             4.3226 | 0.95s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     17.38 |       25.0755 |                             9.0207 | 0.89s  |
| Perturbed-History Exploration (a=1.1)                       |     24.23 |       25.1162 |                             4.2813 | 0.91s  |
| Garbage In, Reward Out (a=0.10)                             |     25.73 |       25.2640 |                             4.0182 | 0.80s  |
| Bootstrapped Thompson Sampling (J=100)                      |     17.66 |       28.2224 |                            16.7306 | 1.02s  |
| Bootstrapped Thompson Sampling (J=500)                      |     17.69 |       28.2561 |                            16.8472 | 4.28s  |
| Garbage In, Reward Out (a=0.33)                             |     21.04 |       28.6989 |                             4.8275 | 1.09s  |
| lil' UCB (δ=0.100)                                          |     19.28 |       28.8759 |                             4.7214 | 0.28s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     17.36 |       28.9676 |                            17.3819 | 8.26s  |
| Bootstrapped Thompson Sampling (J=10)                       |     16.96 |       30.0178 |                            18.1195 | 0.35s  |
| Forced Exploration                                          |     31.25 |       30.1683 |                             5.7161 | 0.06s  |
| Perturbed-History Exploration (a=2.1)                       |     18.80 |       30.7373 |                             5.2197 | 1.02s  |
| lil' UCB (δ=0.010)                                          |     16.77 |       32.6000 |                             5.5344 | 0.29s  |
| Garbage In, Reward Out (a=1.00)                             |     17.31 |       32.8438 |                             5.6154 | 1.14s  |
| Boltzmann-Gumbel Exploration                                |     17.50 |       33.1221 |                             5.5971 | 0.33s  |
| lil' UCB (δ=0.001)                                          |     15.55 |       34.6643 |                             5.9113 | 0.26s  |
| UCB1                                                        |     14.58 |       36.5304 |                             6.3337 | 0.15s  |
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

- ReBoot is very fast and seems to work well on longer horizons, though it seems
  hard to tune for shorter ones; I may have misunderstood how it is supposed to
  be implemented though.

  The paper doesn't specify what exactly the `sigma_a` parameter is, for
  example. Do we need to set that as a hyperparameter or, as I have done, do we
  estimate the standard deviation from rewards received thus far and inflate
  them with `r`?

- As mentioned in the ReBoot paper, the Vanilla Residual Bootstrap does not work
  well on longer horizons, though it does seem to do fairly well on the shorter
  horizons tested here, especially with optimistic initialization.

- ϵ-Exploring Thompson Sampling seems to match or exceed Thompson Sampling
  while being computationally much lighter.
