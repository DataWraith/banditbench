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
| Bootstrapped Thompson Sampling (J=500)                      |     81.55 |       12.3112 |                             1.6871 | 0.74s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     81.44 |       12.6758 |                             1.7121 | 1.35s  |
| TS-UCB (100 samples)                                        |     72.43 |       17.4061 |                             3.2706 | 11.77s |
| TS-UCB (10 samples)                                         |     72.88 |       17.8546 |                             3.5976 | 1.22s  |
| UCB-DT (γ=1.00)                                             |     70.73 |       18.2706 |                             2.5295 | 0.51s  |
| UCB-DT (γ=0.90)                                             |     73.02 |       18.3178 |                             2.4600 | 0.51s  |
| UCB-DT (γ=0.95)                                             |     72.98 |       18.3323 |                             2.4505 | 0.51s  |
| UCB-DT (γ=0.75)                                             |     72.98 |       18.3605 |                             2.4852 | 0.52s  |
| Greedy                                                      |     67.48 |       19.7483 |                             2.4973 | 0.03s  |
| TS-UCB (1 samples)                                          |     72.28 |       19.9767 |                             5.3785 | 0.14s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     63.36 |       21.1298 |                             6.2710 | 4.02s  |
| WR-SDA                                                      |     67.66 |       23.8199 |                             5.0460 | 0.35s  |
| Optimistic Thompson Sampling                                |     69.69 |       25.4924 |                             7.1978 | 0.18s  |
| ϵ-Exploring Thompson Sampling                               |     64.31 |       27.5471 |                             8.9868 | 0.04s  |
| Thompson Sampling                                           |     67.00 |       28.9445 |                             7.1632 | 0.14s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     66.88 |       29.0225 |                             7.0900 | 0.18s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     66.49 |       29.3398 |                             6.9895 | 0.17s  |
| KL-UCB                                                      |     67.57 |       29.6845 |                             7.4867 | 1.49s  |
| ReBoot (r=0.25)                                             |     61.92 |       30.3897 |                             5.2935 | 0.06s  |
| UCB1-Tuned                                                  |     62.81 |       31.7769 |                             3.6345 | 0.07s  |
| Bootstrapped Thompson Sampling (J=100)                      |     51.85 |       32.8060 |                            13.5494 | 0.18s  |
| Vanilla Residual Bootstrap (init=0)                         |     60.73 |       33.1690 |                             5.4193 | 0.05s  |
| Non-Parametric Thompson Sampling                            |     64.59 |       33.8504 |                             7.0679 | 0.74s  |
| ReBoot (r=0.50)                                             |     59.20 |       34.1884 |                             5.8990 | 0.05s  |
| Bounded Dirichlet Sampling                                  |     64.70 |       34.2376 |                             7.1518 | 0.38s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     58.20 |       34.9791 |                             6.9401 | 0.17s  |
| Multiplier Bootstrap-based Exploration                      |     61.60 |       36.1880 |                             4.1319 | 1.06s  |
| Kullback-Leibler Maillard Sampling                          |     60.53 |       37.5467 |                             8.4138 | 0.12s  |
| Perturbed-History Exploration (a=1.1)                       |     57.78 |       37.8970 |                             5.6488 | 0.14s  |
| Garbage In, Reward Out (a=0.10)                             |     58.49 |       38.7874 |                             5.2714 | 0.14s  |
| Bootstrapped Thompson Sampling (J=10)                       |     45.02 |       39.1305 |                            19.4246 | 0.07s  |
| Vanilla Residual Bootstrap (init=1)                         |     60.28 |       40.6878 |                             4.7500 | 0.05s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     44.92 |       44.1840 |                            10.6738 | 0.18s  |
| lil' UCB (δ=0.100)                                          |     52.87 |       44.9486 |                             5.5879 | 0.08s  |
| Tsallis-INF                                                 |     55.26 |       46.5441 |                             5.8549 | 0.21s  |
| Forced Exploration                                          |     63.74 |       46.7869 |                             6.0952 | 0.03s  |
| ReBoot (r=0.90)                                             |     52.97 |       47.4265 |                             6.6595 | 0.05s  |
| Garbage In, Reward Out (a=0.33)                             |     52.54 |       49.4032 |                             5.5249 | 0.18s  |
| Vanilla Residual Bootstrap (init=5)                         |     56.46 |       50.8622 |                             6.0489 | 0.05s  |
| ReBoot (r=1.00)                                             |     50.62 |       52.0364 |                             6.7317 | 0.05s  |
| EB-TCI                                                      |     42.95 |       56.0202 |                            16.1098 | 0.07s  |
| ETC (m=10)                                                  |     48.78 |       56.4409 |                            10.9740 | 0.03s  |
| Perturbed-History Exploration (a=2.1)                       |     48.19 |       56.7164 |                             6.0494 | 0.17s  |
| lil' UCB (δ=0.010)                                          |     44.60 |       62.4201 |                             6.5913 | 0.08s  |
| Garbage In, Reward Out (a=1.00)                             |     43.69 |       66.7268 |                             7.0150 | 0.20s  |
| Boltzmann-Gumbel Exploration                                |     44.52 |       69.1820 |                             6.7076 | 0.08s  |
| ReBoot (r=1.50)                                             |     41.00 |       72.4983 |                             8.1316 | 0.05s  |
| lil' UCB (δ=0.001)                                          |     39.59 |       74.2236 |                             8.0274 | 0.08s  |
| ETC (m=5)                                                   |     28.21 |       79.4626 |                            24.7524 | 0.03s  |
| ReBoot (r=1.70)                                             |     37.90 |       79.8938 |                             9.0403 | 0.05s  |
| ETC (m=20)                                                  |     50.00 |       85.5308 |                            11.8523 | 0.03s  |
| UCB1                                                        |     34.84 |       87.3965 |                            10.1205 | 0.05s  |
| ReBoot (r=2.10)                                             |     32.69 |       93.3431 |                            10.7795 | 0.05s  |
| ETC (m=3)                                                   |     22.50 |       99.6029 |                            27.4581 | 0.03s  |
| ETC (m=25)                                                  |     42.62 |      105.5833 |                            14.8455 | 0.03s  |
| Gradient Bandit                                             |     30.56 |      111.1047 |                            17.4381 | 0.08s  |
| ETC (m=2)                                                   |     20.25 |      111.2784 |                            27.3964 | 0.03s  |
| Gradient Bandit (with baseline)                             |     31.78 |      114.0673 |                            11.6366 | 0.08s  |
| Random                                                      |      9.99 |      205.0580 |                            30.3100 | 0.01s  |
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
| Bootstrapped Thompson Sampling (J=500)                      |     60.67 |       20.0135 |                            10.8059 | 0.67s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     60.81 |       20.0744 |                            10.7234 | 1.31s  |
| UCB-DT (γ=0.90)                                             |     44.11 |       25.7379 |                             7.1522 | 0.53s  |
| UCB-DT (γ=0.95)                                             |     44.07 |       25.7444 |                             7.1627 | 0.53s  |
| UCB-DT (γ=0.75)                                             |     44.20 |       25.7518 |                             7.1508 | 0.53s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     44.83 |       26.7704 |                             8.7872 | 1.83s  |
| TS-UCB (100 samples)                                        |     44.83 |       27.4483 |                             6.6267 | 11.69s |
| Bootstrapped Thompson Sampling (J=100)                      |     44.19 |       27.5108 |                            13.3325 | 0.17s  |
| Greedy                                                      |     39.00 |       28.0151 |                             9.7636 | 0.03s  |
| UCB-DT (γ=1.00)                                             |     39.38 |       28.0689 |                             9.7290 | 0.52s  |
| TS-UCB (10 samples)                                         |     45.12 |       28.1337 |                             6.0061 | 1.19s  |
| Bootstrapped Thompson Sampling (J=10)                       |     41.24 |       28.6171 |                            14.3000 | 0.07s  |
| ϵ-Exploring Thompson Sampling                               |     41.08 |       30.8109 |                             9.0357 | 0.04s  |
| TS-UCB (1 samples)                                          |     42.42 |       31.6765 |                             6.1443 | 0.14s  |
| Forced Exploration                                          |     42.52 |       33.2202 |                             5.6321 | 0.03s  |
| WR-SDA                                                      |     38.17 |       34.3574 |                             7.8687 | 0.56s  |
| UCB1-Tuned                                                  |     39.23 |       36.0362 |                             5.7070 | 0.07s  |
| ReBoot (r=0.25)                                             |     36.27 |       36.8780 |                             8.0532 | 0.05s  |
| Vanilla Residual Bootstrap (init=0)                         |     35.53 |       38.0238 |                             7.8845 | 0.05s  |
| Optimistic Thompson Sampling                                |     37.57 |       38.4989 |                             7.1213 | 0.18s  |
| Multiplier Bootstrap-based Exploration                      |     36.71 |       38.8681 |                             6.9205 | 1.04s  |
| ReBoot (r=0.50)                                             |     34.58 |       39.6438 |                             8.1733 | 0.05s  |
| ETC (m=10)                                                  |     33.76 |       40.4184 |                            12.0078 | 0.03s  |
| Thompson Sampling                                           |     35.68 |       40.6934 |                             7.4756 | 0.14s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     35.61 |       40.7462 |                             7.4738 | 0.18s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     35.54 |       40.8342 |                             7.6058 | 0.17s  |
| Garbage In, Reward Out (a=0.10)                             |     34.26 |       42.3026 |                             7.5669 | 0.17s  |
| Perturbed-History Exploration (a=1.1)                       |     34.15 |       42.4480 |                             7.6337 | 0.15s  |
| KL-UCB                                                      |     35.23 |       42.8489 |                             6.2867 | 1.49s  |
| EB-TCI                                                      |     30.68 |       43.1680 |                             8.8295 | 0.07s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     33.15 |       43.2663 |                             8.0491 | 0.17s  |
| Non-Parametric Thompson Sampling                            |     33.66 |       43.8953 |                             7.4578 | 0.70s  |
| Vanilla Residual Bootstrap (init=1)                         |     33.49 |       43.9511 |                             7.4165 | 0.05s  |
| Bounded Dirichlet Sampling                                  |     33.37 |       44.9539 |                             7.9732 | 0.42s  |
| Tsallis-INF                                                 |     33.02 |       45.9683 |                             8.4113 | 0.22s  |
| lil' UCB (δ=0.100)                                          |     32.27 |       46.6215 |                             6.6925 | 0.07s  |
| Kullback-Leibler Maillard Sampling                          |     30.15 |       48.1212 |                             8.2677 | 0.12s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     27.97 |       48.1233 |                            10.0095 | 0.17s  |
| Garbage In, Reward Out (a=0.33)                             |     30.57 |       48.3822 |                             7.9763 | 0.19s  |
| ReBoot (r=0.90)                                             |     29.75 |       48.7258 |                             8.4139 | 0.05s  |
| ETC (m=5)                                                   |     21.31 |       50.6525 |                            17.7345 | 0.03s  |
| ReBoot (r=1.00)                                             |     28.27 |       51.2697 |                             8.5685 | 0.05s  |
| ETC (m=20)                                                  |     31.27 |       51.6469 |                             8.5980 | 0.03s  |
| Perturbed-History Exploration (a=2.1)                       |     28.34 |       52.5133 |                             8.3130 | 0.17s  |
| Vanilla Residual Bootstrap (init=5)                         |     28.72 |       53.5870 |                             8.3547 | 0.05s  |
| ETC (m=25)                                                  |     32.88 |       56.5539 |                             8.2405 | 0.03s  |
| lil' UCB (δ=0.010)                                          |     26.26 |       57.2169 |                             8.1942 | 0.07s  |
| Garbage In, Reward Out (a=1.00)                             |     25.46 |       58.0798 |                             8.9055 | 0.21s  |
| Boltzmann-Gumbel Exploration                                |     25.93 |       58.3994 |                             8.7698 | 0.07s  |
| ReBoot (r=1.50)                                             |     23.11 |       61.4855 |                             9.5988 | 0.05s  |
| lil' UCB (δ=0.001)                                          |     23.15 |       63.1709 |                             9.1364 | 0.08s  |
| ReBoot (r=1.70)                                             |     21.59 |       64.8451 |                            10.1444 | 0.05s  |
| UCB1                                                        |     20.65 |       68.4993 |                            10.1090 | 0.05s  |
| ReBoot (r=2.10)                                             |     19.33 |       70.1924 |                            10.6578 | 0.05s  |
| ETC (m=3)                                                   |     15.59 |       70.3962 |                            18.2057 | 0.03s  |
| Gradient Bandit                                             |     19.16 |       75.6775 |                            12.1688 | 0.08s  |
| Gradient Bandit (with baseline)                             |     18.70 |       77.4743 |                            10.5750 | 0.07s  |
| ETC (m=2)                                                   |     15.36 |       80.9169 |                            17.7482 | 0.03s  |
| Random                                                      |      9.99 |      102.5290 |                            15.1550 | 0.01s  |
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
| Greedy                                                      |     16.60 |        4.1700 |                             0.0100 | 0.03s  |
| ϵ-Exploring Thompson Sampling                               |     13.52 |        4.3242 |                             0.1100 | 0.04s  |
| Forced Exploration                                          |     13.38 |        4.3312 |                             0.1000 | 0.03s  |
| UCB-DT (γ=0.90)                                             |     13.15 |        4.3424 |                             0.0100 | 0.52s  |
| UCB-DT (γ=0.95)                                             |     13.15 |        4.3424 |                             0.0100 | 0.53s  |
| UCB-DT (γ=1.00)                                             |     13.07 |        4.3464 |                             0.0200 | 0.53s  |
| UCB-DT (γ=0.75)                                             |     12.93 |        4.3535 |                             0.0100 | 0.53s  |
| TS-UCB (100 samples)                                        |     12.06 |        4.3971 |                             0.2500 | 11.71s |
| Bootstrapped Thompson Sampling (J=10)                       |     11.79 |        4.4106 |                             0.1600 | 0.08s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     11.63 |        4.4187 |                             0.3200 | 1.32s  |
| Bootstrapped Thompson Sampling (J=100)                      |     11.61 |        4.4196 |                             0.2900 | 0.18s  |
| Bootstrapped Thompson Sampling (J=500)                      |     11.61 |        4.4196 |                             0.3100 | 0.68s  |
| TS-UCB (10 samples)                                         |     11.58 |        4.4209 |                             0.4400 | 1.19s  |
| EB-TCI                                                      |     11.50 |        4.4250 |                             0.4400 | 0.08s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     11.48 |        4.4258 |                             0.4000 | 0.74s  |
| WR-SDA                                                      |     11.44 |        4.4278 |                             0.3200 | 0.38s  |
| Vanilla Residual Bootstrap (init=0)                         |     11.36 |        4.4320 |                             0.3500 | 0.05s  |
| ReBoot (r=0.25)                                             |     11.32 |        4.4341 |                             0.3500 | 0.06s  |
| ReBoot (r=0.50)                                             |     11.27 |        4.4363 |                             0.3800 | 0.05s  |
| TS-UCB (1 samples)                                          |     11.26 |        4.4368 |                             0.4600 | 0.14s  |
| Optimistic Thompson Sampling                                |     11.26 |        4.4371 |                             0.4400 | 0.18s  |
| Tsallis-INF                                                 |     11.25 |        4.4377 |                             0.2900 | 0.22s  |
| Non-Parametric Thompson Sampling                            |     11.22 |        4.4391 |                             0.4100 | 0.71s  |
| Vanilla Residual Bootstrap (init=1)                         |     11.21 |        4.4393 |                             0.4200 | 0.05s  |
| Thompson Sampling                                           |     11.21 |        4.4397 |                             0.4300 | 0.14s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     11.20 |        4.4398 |                             0.4400 | 0.18s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     11.20 |        4.4401 |                             0.4400 | 0.17s  |
| Perturbed-History Exploration (a=1.1)                       |     11.20 |        4.4402 |                             0.4300 | 0.16s  |
| Multiplier Bootstrap-based Exploration                      |     11.19 |        4.4406 |                             0.4400 | 1.02s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     11.15 |        4.4426 |                             0.4100 | 0.17s  |
| Garbage In, Reward Out (a=0.10)                             |     11.12 |        4.4441 |                             0.3400 | 0.17s  |
| Garbage In, Reward Out (a=0.33)                             |     11.09 |        4.4455 |                             0.3800 | 0.19s  |
| KL-UCB                                                      |     11.06 |        4.4468 |                             0.3000 | 1.49s  |
| ReBoot (r=0.90)                                             |     11.03 |        4.4485 |                             0.3700 | 0.05s  |
| Perturbed-History Exploration (a=2.1)                       |     10.95 |        4.4524 |                             0.3300 | 0.18s  |
| Kullback-Leibler Maillard Sampling                          |     10.94 |        4.4530 |                             0.3300 | 0.12s  |
| ReBoot (r=1.00)                                             |     10.93 |        4.4537 |                             0.3300 | 0.05s  |
| lil' UCB (δ=0.100)                                          |     10.92 |        4.4539 |                             0.2800 | 0.07s  |
| Vanilla Residual Bootstrap (init=5)                         |     10.92 |        4.4540 |                             0.2800 | 0.05s  |
| Bounded Dirichlet Sampling                                  |     10.91 |        4.4545 |                             0.2900 | 0.39s  |
| UCB1-Tuned                                                  |     10.82 |        4.4591 |                             0.4600 | 0.07s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     10.78 |        4.4612 |                             0.3100 | 0.18s  |
| lil' UCB (δ=0.010)                                          |     10.75 |        4.4625 |                             0.2500 | 0.07s  |
| Boltzmann-Gumbel Exploration                                |     10.73 |        4.4636 |                             0.2600 | 0.08s  |
| Garbage In, Reward Out (a=1.00)                             |     10.71 |        4.4646 |                             0.2600 | 0.21s  |
| lil' UCB (δ=0.001)                                          |     10.59 |        4.4707 |                             0.1700 | 0.08s  |
| ReBoot (r=1.50)                                             |     10.56 |        4.4722 |                             0.2000 | 0.05s  |
| ReBoot (r=1.70)                                             |     10.46 |        4.4771 |                             0.1700 | 0.05s  |
| ReBoot (r=2.10)                                             |     10.34 |        4.4829 |                             0.1400 | 0.05s  |
| UCB1                                                        |     10.26 |        4.4872 |                             0.1300 | 0.04s  |
| Gradient Bandit (with baseline)                             |     10.23 |        4.4885 |                             0.1100 | 0.07s  |
| Gradient Bandit                                             |     10.18 |        4.4912 |                             0.1300 | 0.08s  |
| ETC (m=20)                                                  |     10.06 |        4.4970 |                             0.0000 | 0.03s  |
| ETC (m=10)                                                  |     10.01 |        4.4993 |                             0.0000 | 0.03s  |
| Random                                                      |      9.98 |        4.5008 |                             0.0400 | 0.01s  |
| ETC (m=3)                                                   |      9.95 |        4.5027 |                             0.0000 | 0.03s  |
| ETC (m=2)                                                   |      9.94 |        4.5028 |                             0.0000 | 0.03s  |
| ETC (m=5)                                                   |      9.93 |        4.5036 |                             0.0000 | 0.03s  |
| ETC (m=25)                                                  |      9.87 |        4.5065 |                             0.0000 | 0.03s  |
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
| UCB-DT (γ=0.75)                                             |     55.00 |       22.7051 |                             6.0302 | 0.50s  |
| UCB-DT (γ=0.95)                                             |     54.67 |       22.8374 |                             6.0357 | 0.50s  |
| UCB-DT (γ=0.90)                                             |     54.53 |       22.8662 |                             6.0630 | 0.50s  |
| UCB-DT (γ=1.00)                                             |     53.44 |       22.9767 |                             7.3694 | 0.49s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     56.91 |       23.2902 |                             7.1493 | 2.82s  |
| ReBoot (r=0.25)                                             |     52.94 |       24.7856 |                             8.6439 | 0.06s  |
| TS-UCB (100 samples)                                        |     56.19 |       25.1924 |                             4.4774 | 12.37s |
| TS-UCB (10 samples)                                         |     54.99 |       26.7554 |                             4.4802 | 1.26s  |
| ReBoot (r=0.50)                                             |     51.72 |       29.2332 |                             6.5279 | 0.06s  |
| TS-UCB (1 samples)                                          |     52.72 |       29.8275 |                             5.0292 | 0.15s  |
| Bootstrapped Thompson Sampling (J=10)                       |     50.33 |       31.3906 |                             6.7436 | 0.08s  |
| Forced Exploration                                          |     49.27 |       31.6725 |                             8.8808 | 0.03s  |
| Multiplier Bootstrap-based Exploration                      |     49.58 |       32.6088 |                             6.2663 | 1.02s  |
| ϵ-Exploring Thompson Sampling                               |     44.70 |       33.6912 |                            12.4300 | 0.04s  |
| UCB1-Tuned                                                  |     48.78 |       34.1720 |                             5.7265 | 0.08s  |
| Bootstrapped Thompson Sampling (J=100)                      |     47.62 |       34.9846 |                             6.5196 | 0.18s  |
| Garbage In, Reward Out (a=0.10)                             |     47.11 |       35.3159 |                             6.5716 | 0.14s  |
| Bootstrapped Thompson Sampling (J=500)                      |     47.24 |       35.4846 |                             6.5623 | 0.68s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     47.25 |       35.5259 |                             6.5145 | 1.31s  |
| Vanilla Residual Bootstrap (init=1)                         |     47.27 |       35.6021 |                             6.3890 | 0.05s  |
| Optimistic Thompson Sampling                                |     47.54 |       36.0169 |                             6.2395 | 0.20s  |
| Vanilla Residual Bootstrap (init=0)                         |     39.97 |       36.7298 |                            15.5781 | 0.05s  |
| ETC (m=5)                                                   |     40.54 |       37.5338 |                            16.3356 | 0.03s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     45.53 |       38.0235 |                             6.6411 | 0.18s  |
| Thompson Sampling                                           |     45.50 |       38.0338 |                             6.6413 | 0.14s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     45.41 |       38.1336 |                             6.6271 | 0.18s  |
| KL-UCB                                                      |     45.14 |       38.3011 |                             5.9485 | 1.43s  |
| Non-Parametric Thompson Sampling                            |     44.28 |       39.6896 |                             6.8661 | 0.70s  |
| ETC (m=10)                                                  |     40.05 |       39.7511 |                            13.7913 | 0.03s  |
| Greedy                                                      |     37.36 |       39.9645 |                            20.3130 | 0.03s  |
| Bounded Dirichlet Sampling                                  |     44.03 |       40.2371 |                             6.7909 | 0.42s  |
| WR-SDA                                                      |     37.82 |       40.8505 |                            18.3470 | 0.66s  |
| ReBoot (r=0.90)                                             |     42.91 |       41.1146 |                             7.3175 | 0.06s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     41.92 |       41.3247 |                             7.3104 | 0.18s  |
| Kullback-Leibler Maillard Sampling                          |     41.32 |       41.7427 |                             7.4157 | 0.12s  |
| Perturbed-History Exploration (a=1.1)                       |     41.26 |       43.0633 |                             7.6161 | 0.16s  |
| ReBoot (r=1.00)                                             |     41.01 |       43.7015 |                             7.7312 | 0.06s  |
| Garbage In, Reward Out (a=0.33)                             |     39.20 |       45.5334 |                             7.9039 | 0.18s  |
| ETC (m=3)                                                   |     34.12 |       45.6912 |                            27.6862 | 0.03s  |
| ETC (m=20)                                                  |     37.93 |       47.7172 |                            13.6372 | 0.03s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     33.92 |       48.8980 |                             9.5939 | 0.18s  |
| lil' UCB (δ=0.100)                                          |     36.67 |       49.0887 |                             7.5057 | 0.08s  |
| ETC (m=25)                                                  |     37.77 |       52.4211 |                            13.9336 | 0.03s  |
| ETC (m=2)                                                   |     30.16 |       53.2569 |                            29.2436 | 0.03s  |
| ReBoot (r=1.50)                                             |     33.52 |       54.1198 |                            10.7651 | 0.06s  |
| Perturbed-History Exploration (a=2.1)                       |     33.06 |       54.2431 |                             9.6641 | 0.19s  |
| Tsallis-INF                                                 |     32.65 |       55.1568 |                            11.1605 | 0.24s  |
| Vanilla Residual Bootstrap (init=5)                         |     31.59 |       56.9543 |                            10.2200 | 0.05s  |
| ReBoot (r=1.70)                                             |     31.29 |       57.3346 |                            11.9602 | 0.06s  |
| EB-TCI                                                      |     24.85 |       58.9761 |                            22.9968 | 0.08s  |
| Garbage In, Reward Out (a=1.00)                             |     29.72 |       58.9769 |                            11.3139 | 0.22s  |
| Boltzmann-Gumbel Exploration                                |     30.21 |       59.0762 |                            11.4529 | 0.08s  |
| lil' UCB (δ=0.010)                                          |     29.49 |       59.3792 |                            11.2005 | 0.08s  |
| ReBoot (r=2.10)                                             |     27.81 |       62.5734 |                            14.2399 | 0.06s  |
| lil' UCB (δ=0.001)                                          |     25.59 |       65.3146 |                            14.4606 | 0.08s  |
| UCB1                                                        |     22.44 |       70.4627 |                            16.8609 | 0.05s  |
| Gradient Bandit                                             |     20.43 |       75.0125 |                            17.3070 | 0.08s  |
| Gradient Bandit (with baseline)                             |     20.06 |       75.7085 |                            17.5892 | 0.08s  |
| Random                                                      |      9.99 |       94.2791 |                            25.9206 | 0.01s  |
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
| TS-UCB (100 samples)                                        |     58.71 |        7.4481 |                             2.1886 | 11.76s |
| TS-UCB (10 samples)                                         |     57.79 |        7.8999 |                             1.9148 | 1.19s  |
| TS-UCB (1 samples)                                          |     57.53 |        8.3487 |                             1.7839 | 0.14s  |
| UCB-DT (γ=1.00)                                             |     55.22 |        8.6731 |                             1.5458 | 0.53s  |
| UCB-DT (γ=0.90)                                             |     55.32 |        8.7670 |                             1.5465 | 0.52s  |
| UCB-DT (γ=0.95)                                             |     55.25 |        8.7822 |                             1.5484 | 0.53s  |
| Greedy                                                      |     53.46 |        8.8426 |                             1.5877 | 0.03s  |
| UCB-DT (γ=0.75)                                             |     55.50 |        8.8734 |                             1.5938 | 0.53s  |
| Optimistic Thompson Sampling                                |     55.57 |        9.3600 |                             3.3451 | 0.18s  |
| WR-SDA                                                      |     52.20 |       10.4022 |                             2.8202 | 0.21s  |
| ϵ-Exploring Thompson Sampling                               |     44.32 |       11.1621 |                             4.2373 | 0.04s  |
| KL-UCB                                                      |     51.72 |       11.7591 |                             3.6041 | 1.21s  |
| Thompson Sampling                                           |     48.36 |       12.6305 |                             2.8003 | 0.14s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     36.88 |       12.6832 |                             4.2582 | 2.43s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     48.28 |       12.7174 |                             2.8361 | 0.18s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     46.43 |       13.2106 |                             2.8578 | 0.18s  |
| Non-Parametric Thompson Sampling                            |     47.42 |       13.7743 |                             4.3390 | 0.71s  |
| Forced Exploration                                          |     48.03 |       14.2409 |                             2.5482 | 0.03s  |
| Bounded Dirichlet Sampling                                  |     45.50 |       14.7444 |                             4.6974 | 0.37s  |
| Kullback-Leibler Maillard Sampling                          |     43.49 |       15.3254 |                             5.1663 | 0.11s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     27.59 |       18.2837 |                             5.3096 | 0.18s  |
| EB-TCI                                                      |     35.83 |       20.0130 |                             5.2114 | 0.08s  |
| ReBoot (r=0.25)                                             |     34.86 |       20.2055 |                             3.2261 | 0.06s  |
| Vanilla Residual Bootstrap (init=0)                         |     33.47 |       21.7507 |                             3.2564 | 0.05s  |
| Multiplier Bootstrap-based Exploration                      |     28.54 |       22.5267 |                             3.5576 | 1.02s  |
| ETC (m=20)                                                  |     33.67 |       22.5570 |                             4.2197 | 0.03s  |
| ReBoot (r=0.50)                                             |     30.85 |       22.7688 |                             3.8323 | 0.06s  |
| ETC (m=10)                                                  |     26.78 |       22.8167 |                             6.4075 | 0.03s  |
| UCB1-Tuned                                                  |     25.26 |       23.1257 |                             3.4924 | 0.08s  |
| Vanilla Residual Bootstrap (init=1)                         |     31.25 |       23.4112 |                             3.3759 | 0.05s  |
| Tsallis-INF                                                 |     26.49 |       23.5590 |                             4.3226 | 0.21s  |
| Vanilla Residual Bootstrap (init=5)                         |     30.55 |       24.1464 |                             3.5251 | 0.05s  |
| Garbage In, Reward Out (a=0.10)                             |     26.92 |       24.2118 |                             3.9104 | 0.15s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     17.38 |       25.0755 |                             9.0207 | 0.17s  |
| Perturbed-History Exploration (a=1.1)                       |     24.23 |       25.1162 |                             4.2813 | 0.17s  |
| ETC (m=25)                                                  |     28.63 |       27.3236 |                             5.1674 | 0.03s  |
| Bootstrapped Thompson Sampling (J=100)                      |     17.66 |       28.2224 |                            16.7306 | 0.18s  |
| Bootstrapped Thompson Sampling (J=500)                      |     17.69 |       28.2561 |                            16.8472 | 0.68s  |
| Garbage In, Reward Out (a=0.33)                             |     21.30 |       28.3366 |                             4.7374 | 0.20s  |
| ReBoot (r=0.90)                                             |     24.03 |       28.5844 |                             5.1178 | 0.05s  |
| lil' UCB (δ=0.100)                                          |     19.28 |       28.8759 |                             4.7214 | 0.08s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     17.36 |       28.9676 |                            17.3819 | 1.32s  |
| Bootstrapped Thompson Sampling (J=10)                       |     16.96 |       30.0178 |                            18.1195 | 0.07s  |
| ReBoot (r=1.00)                                             |     22.47 |       30.1561 |                             5.4255 | 0.05s  |
| Perturbed-History Exploration (a=2.1)                       |     18.80 |       30.7373 |                             5.2197 | 0.19s  |
| lil' UCB (δ=0.010)                                          |     16.77 |       32.6000 |                             5.5344 | 0.08s  |
| Garbage In, Reward Out (a=1.00)                             |     17.33 |       32.8421 |                             5.6612 | 0.22s  |
| Boltzmann-Gumbel Exploration                                |     17.50 |       33.1221 |                             5.5971 | 0.08s  |
| lil' UCB (δ=0.001)                                          |     15.55 |       34.6643 |                             5.9113 | 0.08s  |
| ReBoot (r=1.50)                                             |     18.16 |       35.6912 |                             6.5617 | 0.06s  |
| UCB1                                                        |     14.58 |       36.5304 |                             6.3337 | 0.05s  |
| ReBoot (r=1.70)                                             |     17.19 |       37.2245 |                             6.9281 | 0.05s  |
| ReBoot (r=2.10)                                             |     15.84 |       39.6794 |                             7.4686 | 0.05s  |
| Gradient Bandit                                             |     13.75 |       39.9529 |                             8.1144 | 0.08s  |
| Gradient Bandit (with baseline)                             |     13.20 |       41.3526 |                             7.4311 | 0.08s  |
| ETC (m=5)                                                   |     12.31 |       42.3407 |                             9.2207 | 0.03s  |
| ETC (m=3)                                                   |     11.97 |       44.3016 |                             9.5583 | 0.03s  |
| ETC (m=2)                                                   |     11.02 |       45.7899 |                             9.4387 | 0.04s  |
| Random                                                      |      9.97 |       49.8281 |                             9.9126 | 0.01s  |
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
