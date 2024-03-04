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

- Random Baseline (chooses arms randomly)
- Greedy Baseline (chooses the arm with the maximum average reward)
- [Boltzmann-Gumbel Exploration](https://arxiv.org/abs/1705.10257)
- [Bounded Dirichlet Sampling](https://arxiv.org/abs/2111.09724)
- [EB-TCI](https://arxiv.org/abs/2206.05979)
- [ϵ-Exploring Thompson Sampling](https://proceedings.mlr.press/v202/jin23b/jin23b.pdf) (PDF)
- [Forced Exploration (Linear)](https://arxiv.org/abs/2312.07285)
- [Garbage In, Reward Out](http://proceedings.mlr.press/v97/kveton19a/kveton19a.pdf) (PDF)
- [Gradient Bandit](https://arxiv.org/abs/2402.17235)
- [KL-UCB](https://arxiv.org/abs/1102.2490)
- [Kullback-Leibler Maillard Sampling](https://arxiv.org/abs/2304.14989)
- [Multiplier Bootstrap-based Exploration](https://arxiv.org/abs/2302.01543)
- [Non-Parametric Thompson Sampling](https://proceedings.mlr.press/v117/riou20a.html)
- [Perturbed-History Exploration](https://arxiv.org/abs/1902.10089)
- [ReBoot](https://arxiv.org/abs/2002.08436)
- [Satisficing Thompson Sampling](https://arxiv.org/abs/1704.09028)
- [Thompson Sampling with Virtual Helping Agents (C3)](https://arxiv.org/abs/2209.08197)
- Thompson Sampling
- [TS-UCB](https://arxiv.org/abs/2006.06372)
- [UCB1](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [UCB1-Tuned](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [WR-SDA](https://arxiv.org/abs/2010.14323)

## Experiments

### Uniform

This experiment uses 10 arms, with the means sampled uniformly from the interval
[0, 1]. This is a relatively easy instance, because there is likely to be a
single best arm that is easy to find. This is reflected in the %-Optimal column,
where the best algorithms reach over 2/3 pull rate of the optimal arm.

<details>
<summary>Results</summary>

<!-- `> cargo run --release --bin uniform` -->
<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| Vanilla Residual Bootstrap (init=1)                         |    73.31% |       16.9644 |                             3.0372 | 0.22s  |
| TS-UCB (100 samples)                                        |    72.43% |       17.4061 |                             3.2706 | 66.09s |
| TS-UCB (10 samples)                                         |    72.88% |       17.8546 |                             3.5976 | 6.56s  |
| Vanilla Residual Bootstrap (init=0)                         |    70.53% |       18.4180 |                             2.5125 | 0.23s  |
| ReBoot (r=0.50)                                             |    69.90% |       18.5399 |                             2.5231 | 0.21s  |
| ReBoot (r=0.90)                                             |    70.89% |       19.0026 |                             2.8707 | 0.21s  |
| ReBoot (r=1.00)                                             |    70.75% |       19.6511 |                             2.9337 | 0.21s  |
| Greedy                                                      |    67.48% |       19.7483 |                             2.4973 | 0.11s  |
| TS-UCB (1 sample)                                           |    72.28% |       19.9767 |                             5.3785 | 0.81s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    63.36% |       21.1298 |                             6.2710 | 25.17s |
| WR-SDA                                                      |    67.66% |       23.8199 |                             5.0460 | 1.64s  |
| Multiplier Bootstrap-based Exploration                      |    67.82% |       26.0614 |                             3.6393 | 5.98s  |
| ReBoot (r=1.50)                                             |    70.37% |       26.4939 |                             3.5531 | 0.22s  |
| ϵ-Exploring Thompson Sampling                               |    64.31% |       27.5471 |                             8.9868 | 0.19s  |
| Thompson Sampling                                           |    67.00% |       28.9445 |                             7.1632 | 0.75s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |    66.88% |       29.0225 |                             7.0900 | 0.95s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |    66.49% |       29.3398 |                             6.9895 | 0.97s  |
| KL-UCB                                                      |    67.56% |       29.6893 |                             7.4957 | 7.70s  |
| ReBoot (r=1.70)                                             |    68.47% |       31.4177 |                             3.6511 | 0.22s  |
| UCB1-Tuned                                                  |    62.81% |       31.7769 |                             3.6345 | 0.29s  |
| Non-Parametric Thompson Sampling                            |    64.59% |       33.8504 |                             7.0679 | 4.52s  |
| Bounded Dirichlet Sampling                                  |    64.70% |       34.2376 |                             7.1518 | 2.23s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |    58.20% |       34.9791 |                             6.9401 | 0.97s  |
| Kullback-Leibler Maillard Sampling                          |    60.53% |       37.5467 |                             8.4138 | 0.53s  |
| Perturbed-History Exploration (a=1.1)                       |    57.78% |       37.8970 |                             5.6488 | 0.82s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |    44.92% |       44.1840 |                            10.6738 | 0.96s  |
| Garbage In, Reward Out (a=0.10)                             |    57.08% |       44.4496 |                             4.8697 | 0.95s  |
| Garbage In, Reward Out (a=0.33)                             |    51.88% |       51.5502 |                             5.3784 | 1.09s  |
| EB-TCI                                                      |    42.95% |       56.0202 |                            16.1098 | 0.34s  |
| Perturbed-History Exploration (a=2.1)                       |    48.19% |       56.7164 |                             6.0494 | 0.91s  |
| Garbage In, Reward Out (a=1.00)                             |    43.64% |       66.8026 |                             7.0771 | 1.30s  |
| Boltzmann-Gumbel Exploration                                |    44.52% |       69.1820 |                             6.7076 | 0.43s  |
| UCB1                                                        |    34.84% |       87.3965 |                            10.1205 | 0.16s  |
| Gradient Bandit                                             |    30.56% |      111.1047 |                            17.4381 | 0.42s  |
| Gradient Bandit (with baseline)                             |    31.78% |      114.0673 |                            11.6366 | 0.42s  |
| Forced Exploration                                          |    39.67% |      120.7367 |                            16.8185 | 0.10s  |
| Random                                                      |     9.99% |      205.0580 |                            30.3100 | 0.03s  |
<!-- END mdsh -->

</details>

### Half-Range

This experiment uses 10 arms, with the means sampled uniformly from the interval
\[0.25, 0.75\]. This is a harder instance, because the arms are closer together
and thus harder to distinguish.

This experiment was taken from the GIRO paper.

<details>
<summary>Results</summary>

<!-- `> cargo run --release --bin half_range` -->
<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| Vanilla Residual Bootstrap (init=1)                         |    45.94% |       24.6010 |                             6.5389 | 0.23s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    44.83% |       26.7704 |                             8.7872 | 11.81s |
| TS-UCB (100 samples)                                        |    44.83% |       27.4483 |                             6.6267 | 66.37s |
| Vanilla Residual Bootstrap (init=0)                         |    39.98% |       27.7827 |                             9.2352 | 0.23s  |
| ReBoot (r=1.00)                                             |    41.18% |       27.8871 |                             8.3985 | 0.24s  |
| Greedy                                                      |    39.00% |       28.0151 |                             9.7636 | 0.15s  |
| ReBoot (r=0.90)                                             |    40.63% |       28.0172 |                             8.8135 | 0.25s  |
| ReBoot (r=0.50)                                             |    39.52% |       28.0805 |                             9.6491 | 0.22s  |
| TS-UCB (10 samples)                                         |    45.12% |       28.1337 |                             6.0061 | 6.77s  |
| ϵ-Exploring Thompson Sampling                               |    41.08% |       30.8109 |                             9.0357 | 0.17s  |
| Multiplier Bootstrap-based Exploration                      |    42.47% |       30.9818 |                             6.6402 | 5.97s  |
| TS-UCB (1 sample)                                           |    42.42% |       31.6765 |                             6.1443 | 0.70s  |
| ReBoot (r=1.50)                                             |    42.27% |       31.7111 |                             6.1746 | 0.26s  |
| WR-SDA                                                      |    38.17% |       34.3574 |                             7.8687 | 2.57s  |
| ReBoot (r=1.70)                                             |    39.81% |       35.3730 |                             6.1512 | 0.25s  |
| UCB1-Tuned                                                  |    39.23% |       36.0362 |                             5.7070 | 0.30s  |
| Thompson Sampling                                           |    35.68% |       40.6934 |                             7.4756 | 0.65s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |    35.61% |       40.7462 |                             7.4738 | 0.95s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |    35.54% |       40.8342 |                             7.6058 | 0.97s  |
| Perturbed-History Exploration (a=1.1)                       |    34.15% |       42.4480 |                             7.6337 | 0.97s  |
| KL-UCB                                                      |    35.22% |       42.8549 |                             6.2878 | 8.26s  |
| EB-TCI                                                      |    30.68% |       43.1680 |                             8.8295 | 0.36s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |    33.15% |       43.2663 |                             8.0491 | 0.98s  |
| Non-Parametric Thompson Sampling                            |    33.66% |       43.8953 |                             7.4578 | 4.52s  |
| Bounded Dirichlet Sampling                                  |    33.37% |       44.9539 |                             7.9732 | 2.67s  |
| Garbage In, Reward Out (a=0.10)                             |    32.82% |       44.9909 |                             7.5012 | 1.02s  |
| Kullback-Leibler Maillard Sampling                          |    30.15% |       48.1212 |                             8.2677 | 0.65s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |    27.97% |       48.1233 |                            10.0095 | 1.02s  |
| Garbage In, Reward Out (a=0.33)                             |    30.19% |       49.2192 |                             8.0236 | 1.12s  |
| Perturbed-History Exploration (a=2.1)                       |    28.34% |       52.5133 |                             8.3130 | 1.01s  |
| Garbage In, Reward Out (a=1.00)                             |    25.47% |       58.0660 |                             8.8999 | 1.35s  |
| Boltzmann-Gumbel Exploration                                |    25.93% |       58.3994 |                             8.7698 | 0.41s  |
| Forced Exploration                                          |    27.38% |       65.0601 |                             9.4003 | 0.09s  |
| UCB1                                                        |    20.65% |       68.4993 |                            10.1090 | 0.17s  |
| Gradient Bandit                                             |    19.16% |       75.6775 |                            12.1688 | 0.43s  |
| Gradient Bandit (with baseline)                             |    18.70% |       77.4743 |                            10.5750 | 0.48s  |
| Random                                                      |     9.99% |      102.5290 |                            15.1550 | 0.03s  |
<!-- END mdsh -->

</details>

### Hard

This experiment uses 10 arms. All arms have a success probability of 0.5, except
for the best arm, which has a success probability of 0.51.

This experiment was taken from the paper describing Boltzmann-Gumbel Exploration.

<details>
<summary>Results</summary>

<!-- `> cargo run --release --bin hard` -->
<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| Greedy                                                      |    16.72% |        4.1640 |                             0.1100 | 0.13s  |
| Vanilla Residual Bootstrap (init=0)                         |    14.09% |        4.2955 |                             0.1100 | 0.20s  |
| ϵ-Exploring Thompson Sampling                               |    13.51% |        4.3245 |                             0.1100 | 0.22s  |
| ReBoot (r=0.50)                                             |    13.12% |        4.3439 |                             0.1200 | 0.25s  |
| Forced Exploration                                          |    13.03% |        4.3486 |                             0.1900 | 0.11s  |
| Vanilla Residual Bootstrap (init=1)                         |    12.84% |        4.3578 |                             0.1700 | 0.23s  |
| ReBoot (r=0.90)                                             |    12.64% |        4.3682 |                             0.1300 | 0.26s  |
| ReBoot (r=1.00)                                             |    12.48% |        4.3759 |                             0.1400 | 0.25s  |
| TS-UCB (100 samples)                                        |    12.05% |        4.3973 |                             0.2500 | 67.17s |
| ReBoot (r=1.50)                                             |    11.56% |        4.4222 |                             0.2400 | 0.26s  |
| EB-TCI                                                      |    11.55% |        4.4225 |                             0.4400 | 0.43s  |
| TS-UCB (10 samples)                                         |    11.55% |        4.4227 |                             0.2400 | 6.84s  |
| Multiplier Bootstrap-based Exploration                      |    11.47% |        4.4263 |                             0.2500 | 6.08s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    11.45% |        4.4273 |                             0.2600 | 4.66s  |
| WR-SDA                                                      |    11.45% |        4.4275 |                             0.3200 | 1.75s  |
| ReBoot (r=1.70)                                             |    11.28% |        4.4361 |                             0.3300 | 0.26s  |
| TS-UCB (1 sample)                                           |    11.21% |        4.4394 |                             0.4300 | 0.78s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |    11.17% |        4.4414 |                             0.4200 | 0.97s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |    11.16% |        4.4418 |                             0.4100 | 0.96s  |
| Non-Parametric Thompson Sampling                            |    11.16% |        4.4418 |                             0.4000 | 4.49s  |
| Perturbed-History Exploration (a=1.1)                       |    11.15% |        4.4425 |                             0.4200 | 1.02s  |
| Garbage In, Reward Out (a=0.10)                             |    11.15% |        4.4426 |                             0.4100 | 1.15s  |
| Thompson Sampling                                           |    11.15% |        4.4427 |                             0.4200 | 0.74s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |    11.07% |        4.4464 |                             0.4000 | 1.01s  |
| Garbage In, Reward Out (a=0.33)                             |    11.05% |        4.4477 |                             0.3800 | 1.32s  |
| KL-UCB                                                      |    11.02% |        4.4490 |                             0.2300 | 8.13s  |
| Kullback-Leibler Maillard Sampling                          |    10.93% |        4.4533 |                             0.3400 | 0.64s  |
| Perturbed-History Exploration (a=2.1)                       |    10.92% |        4.4539 |                             0.3300 | 1.16s  |
| Bounded Dirichlet Sampling                                  |    10.86% |        4.4572 |                             0.2900 | 2.43s  |
| UCB1-Tuned                                                  |    10.76% |        4.4620 |                             0.4400 | 0.27s  |
| Garbage In, Reward Out (a=1.00)                             |    10.69% |        4.4656 |                             0.2600 | 1.34s  |
| Boltzmann-Gumbel Exploration                                |    10.68% |        4.4660 |                             0.2600 | 0.40s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |    10.68% |        4.4661 |                             0.3100 | 1.03s  |
| UCB1                                                        |    10.24% |        4.4880 |                             0.1600 | 0.17s  |
| Gradient Bandit (with baseline)                             |    10.20% |        4.4899 |                             0.1100 | 0.44s  |
| Gradient Bandit                                             |    10.18% |        4.4908 |                             0.1300 | 0.43s  |
| Random                                                      |     9.98% |        4.5009 |                             0.0500 | 0.03s  |
<!-- END mdsh -->

</details>

### Beta

This experiment uses 10 arms. The arm means are sampled from a Beta(1, 8) distribution.

This experiment was taken from the paper *Multiplier Bootstrap-based Exploration*.

<details>
<summary>Results</summary>

<!-- `> cargo run --release --bin beta` -->
<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| Vanilla Residual Bootstrap (init=1)                         |    56.71% |       22.3221 |                             4.6914 | 0.26s  |
| ReBoot (r=1.00)                                             |    55.00% |       22.7015 |                             5.7422 | 0.29s  |
| ReBoot (r=0.90)                                             |    53.45% |       23.1910 |                             6.5038 | 0.31s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    56.91% |       23.2902 |                             7.1493 | 19.60s |
| TS-UCB (100 samples)                                        |    56.19% |       25.1924 |                             4.4774 | 74.36s |
| ReBoot (r=1.50)                                             |    55.33% |       25.5983 |                             4.8770 | 0.28s  |
| Multiplier Bootstrap-based Exploration                      |    54.92% |       25.7531 |                             5.7460 | 6.70s  |
| TS-UCB (10 samples)                                         |    54.99% |       26.7554 |                             4.4802 | 7.73s  |
| ReBoot (r=1.70)                                             |    54.15% |       27.8226 |                             5.3817 | 0.29s  |
| TS-UCB (1 sample)                                           |    52.72% |       29.8275 |                             5.0292 | 0.92s  |
| ReBoot (r=0.50)                                             |    44.19% |       30.2711 |                            12.3522 | 0.32s  |
| ϵ-Exploring Thompson Sampling                               |    44.70% |       33.6912 |                            12.4300 | 0.17s  |
| UCB1-Tuned                                                  |    48.78% |       34.1720 |                             5.7265 | 0.34s  |
| Garbage In, Reward Out (a=0.10)                             |    46.27% |       36.5880 |                             6.6192 | 0.89s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |    45.53% |       38.0235 |                             6.6411 | 1.12s  |
| Thompson Sampling                                           |    45.50% |       38.0338 |                             6.6413 | 0.88s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |    45.41% |       38.1336 |                             6.6271 | 1.11s  |
| KL-UCB                                                      |    45.13% |       38.3085 |                             5.9510 | 8.64s  |
| Vanilla Residual Bootstrap (init=0)                         |    38.13% |       39.3544 |                            19.2688 | 0.26s  |
| Non-Parametric Thompson Sampling                            |    44.28% |       39.6896 |                             6.8661 | 5.12s  |
| Greedy                                                      |    37.36% |       39.9645 |                            20.3130 | 0.13s  |
| Bounded Dirichlet Sampling                                  |    44.03% |       40.2371 |                             6.7909 | 2.42s  |
| WR-SDA                                                      |    37.82% |       40.8505 |                            18.3470 | 3.22s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |    41.92% |       41.3247 |                             7.3104 | 1.17s  |
| Kullback-Leibler Maillard Sampling                          |    41.32% |       41.7427 |                             7.4157 | 0.63s  |
| Perturbed-History Exploration (a=1.1)                       |    41.26% |       43.0633 |                             7.6161 | 1.10s  |
| Garbage In, Reward Out (a=0.33)                             |    38.72% |       46.2679 |                             7.9517 | 1.08s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |    33.92% |       48.8980 |                             9.5939 | 1.20s  |
| Perturbed-History Exploration (a=2.1)                       |    33.06% |       54.2431 |                             9.6641 | 1.44s  |
| Forced Exploration                                          |    33.93% |       58.8258 |                            16.0080 | 0.09s  |
| EB-TCI                                                      |    24.85% |       58.9761 |                            22.9968 | 0.32s  |
| Garbage In, Reward Out (a=1.00)                             |    29.74% |       58.9955 |                            11.3563 | 1.17s  |
| Boltzmann-Gumbel Exploration                                |    30.21% |       59.0762 |                            11.4529 | 0.35s  |
| UCB1                                                        |    22.44% |       70.4627 |                            16.8609 | 0.21s  |
| Gradient Bandit                                             |    20.43% |       75.0125 |                            17.3070 | 0.44s  |
| Gradient Bandit (with baseline)                             |    20.06% |       75.7085 |                            17.5892 | 0.46s  |
| Random                                                      |     9.99% |       94.2791 |                            25.9206 | 0.05s  |
<!-- END mdsh -->

</details>

### Reverse Beta

This experiment uses 10 arms. The arm means are sampled from a Beta(8, 1) distribution.

I added this to see which algorithms are affected by rewards close to 1 instead of close to 0.

<details>
<summary>Results</summary>

<!-- `> cargo run --release --bin reverse_beta` -->
<!-- BEGIN mdsh -->
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) |  Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :----: |
| TS-UCB (100 samples)                                        |    58.71% |        7.4481 |                             2.1886 | 70.31s |
| TS-UCB (10 samples)                                         |    57.79% |        7.8999 |                             1.9148 | 7.11s  |
| TS-UCB (1 sample)                                           |    57.53% |        8.3487 |                             1.7839 | 0.76s  |
| Vanilla Residual Bootstrap (init=1)                         |    54.29% |        8.6730 |                             1.5690 | 0.24s  |
| ReBoot (r=0.50)                                             |    53.85% |        8.7544 |                             1.5784 | 0.25s  |
| Vanilla Residual Bootstrap (init=0)                         |    53.82% |        8.7563 |                             1.5834 | 0.23s  |
| ReBoot (r=0.90)                                             |    54.52% |        8.8017 |                             1.5919 | 0.26s  |
| Greedy                                                      |    53.46% |        8.8426 |                             1.5877 | 0.13s  |
| ReBoot (r=1.00)                                             |    54.58% |        8.9873 |                             1.6223 | 0.25s  |
| WR-SDA                                                      |    52.20% |       10.4022 |                             2.8202 | 1.05s  |
| ϵ-Exploring Thompson Sampling                               |    44.32% |       11.1621 |                             4.2373 | 0.17s  |
| KL-UCB                                                      |    51.72% |       11.7599 |                             3.6028 | 6.71s  |
| Thompson Sampling                                           |    48.36% |       12.6305 |                             2.8003 | 0.75s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    36.88% |       12.6832 |                             4.2582 | 15.13s |
| ReBoot (r=1.50)                                             |    50.83% |       12.6931 |                             2.2936 | 0.23s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |    48.28% |       12.7174 |                             2.8361 | 1.01s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |    46.43% |       13.2106 |                             2.8578 | 1.05s  |
| Non-Parametric Thompson Sampling                            |    47.42% |       13.7743 |                             4.3390 | 4.83s  |
| Bounded Dirichlet Sampling                                  |    45.50% |       14.7444 |                             4.6974 | 2.37s  |
| ReBoot (r=1.70)                                             |    48.26% |       14.9293 |                             2.5980 | 0.26s  |
| Kullback-Leibler Maillard Sampling                          |    43.49% |       15.3254 |                             5.1663 | 0.62s  |
| Multiplier Bootstrap-based Exploration                      |    37.02% |       17.2756 |                             2.6160 | 6.38s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |    27.59% |       18.2837 |                             5.3096 | 1.01s  |
| EB-TCI                                                      |    35.83% |       20.0130 |                             5.2114 | 0.35s  |
| UCB1-Tuned                                                  |    25.26% |       23.1257 |                             3.4924 | 0.33s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |    17.38% |       25.0755 |                             9.0207 | 0.99s  |
| Perturbed-History Exploration (a=1.1)                       |    24.23% |       25.1162 |                             4.2813 | 1.15s  |
| Garbage In, Reward Out (a=0.10)                             |    25.73% |       25.2640 |                             4.0182 | 1.00s  |
| Garbage In, Reward Out (a=0.33)                             |    21.04% |       28.6989 |                             4.8275 | 1.38s  |
| Forced Exploration                                          |    31.25% |       30.1683 |                             5.7161 | 0.10s  |
| Perturbed-History Exploration (a=2.1)                       |    18.80% |       30.7373 |                             5.2197 | 1.16s  |
| Garbage In, Reward Out (a=1.00)                             |    17.31% |       32.8438 |                             5.6154 | 1.28s  |
| Boltzmann-Gumbel Exploration                                |    17.50% |       33.1221 |                             5.5971 | 0.41s  |
| UCB1                                                        |    14.58% |       36.5304 |                             6.3337 | 0.19s  |
| Gradient Bandit                                             |    13.75% |       39.9529 |                             8.1144 | 0.46s  |
| Gradient Bandit (with baseline)                             |    13.20% |       41.3526 |                             7.4311 | 0.48s  |
| Random                                                      |     9.97% |       49.8281 |                             9.9126 | 0.05s  |
<!-- END mdsh -->

</details>

## Notes / Conclusions

- Keep in mind that the experiments were on the Bernoulli Bandit with a short
  horizon only. Many of the algorithms also work on other kinds of bandits and
  may rank differently there.

- TS-UCB seems to be the best algorithm overall; when extending the time horizon
  beyond 500 steps (not shown above), it almost always comes out on top, but
  even on short horizons it does relatively well. It is also very flexible with
  regard to the number of samples, and thus the amount of time it takes to run it.

- ReBoot seems to work well on longer horizons, though it seems hard to tune
  for shorter ones; I may have misunderstood how it is supposed to be implemented
  though.

  The paper doesn't specify what exactly the `sigma_a` parameter is, for
  example. Do we need to set that as a hyperparameter or, as I have done, do we
  estimate the standard deviation from rewards received thus far and inflate
  them with `r`?

- As mentioned in the ReBoot paper, the Vanilla Residual Bootstrap does not work
  well on longer horizons, though it does seem to do fairly well on the shorter
  horizons tested here, especially with optimistic initialization.

- Greedy algorithms seem to do well on a short horizon in general, but fall
  behind after a few hundred steps.

- ϵ-Exploring Thompson Sampling seems to match or exceed Thompson Sampling
  while being computationally much lighter.

- I have probably made mistakes while translating the formulas from the papers
  into code – KL-UCB, PHE, GIRO, etc. are doing much worse than I expected.
