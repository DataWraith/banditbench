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
- [Non-Parametric Thompson Sampling](https://proceedings.mlr.press/v117/riou20a.html)
- Optimistic Thompson Sampling
- [Satisficing Thompson Sampling](https://arxiv.org/abs/1704.09028)
- [Thompson Sampling with Virtual Helping Agents (C3)](https://arxiv.org/abs/2209.08197)
- Thompson Sampling

### Upper Confidence Bound-based

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
- [EXP-IX](https://arxiv.org/pdf/1506.03271)
- [Forced Exploration](https://arxiv.org/abs/2312.07285)
- [Gradient Bandit](https://arxiv.org/abs/2402.17235)
- [Kullback-Leibler Maillard Sampling](https://arxiv.org/abs/2304.14989)
- [POKER](https://link.springer.com/chapter/10.1007/11564096_42) (with fixed Horizon)
- [Tsallis-INF](https://arxiv.org/abs/1807.07623)
- [TS-UCB](https://arxiv.org/abs/2006.06372)

## Experiments

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
| TS-UCB (100 samples)                                        |     72.02 |       16.8866 |                             3.3722 | 11.26s |
| TS-UCB (10 samples)                                         |     72.48 |       17.2679 |                             3.7317 | 1.19s  |
| UCB-DT (γ=1.00)                                             |     69.93 |       18.1466 |                             2.5287 | 0.52s  |
| UCB-DT (γ=0.95)                                             |     72.44 |       18.1946 |                             2.4725 | 0.52s  |
| UCB-DT (γ=0.75)                                             |     72.50 |       18.1962 |                             2.5172 | 0.53s  |
| UCB-DT (γ=0.90)                                             |     72.42 |       18.2016 |                             2.4807 | 0.53s  |
| MOSS-Anytime (α=-0.85)                                      |     69.71 |       18.8113 |                             2.5659 | 0.05s  |
| CODE (δ=0.990)                                              |     68.91 |       18.9329 |                             2.9569 | 0.08s  |
| POKER (H=10)                                                |     65.48 |       19.3526 |                             3.2000 | 0.09s  |
| POKER (H=5)                                                 |     66.34 |       19.3558 |                             2.7035 | 0.09s  |
| TS-UCB (1 samples)                                          |     71.83 |       19.5545 |                             5.3564 | 0.17s  |
| POKER (H=1)                                                 |     66.55 |       19.5748 |                             2.5553 | 0.09s  |
| Greedy                                                      |     66.26 |       19.7129 |                             2.5470 | 0.02s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     66.35 |       20.7765 |                             2.7735 | 0.03s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     63.16 |       21.1041 |                             6.1932 | 4.14s  |
| ϵ-Greedy (ϵ=0.010)                                          |     66.18 |       21.1769 |                             2.8588 | 0.02s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     66.48 |       21.2824 |                             2.8492 | 0.03s  |
| POKER (H=25)                                                |     61.38 |       21.3386 |                             6.6186 | 0.09s  |
| MOSS-Anytime (α=-0.50)                                      |     70.74 |       22.4582 |                             2.7088 | 0.05s  |
| ϵ-Greedy (ϵ=0.020)                                          |     65.99 |       22.7752 |                             3.1672 | 0.02s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     66.55 |       23.6847 |                             3.3687 | 0.03s  |
| WR-SDA                                                      |     66.87 |       23.8280 |                             5.0922 | 0.36s  |
| MOSS-Anytime (α=-0.33)                                      |     69.75 |       24.4536 |                             2.6909 | 0.05s  |
| Optimistic Thompson Sampling                                |     68.80 |       25.6235 |                             7.1784 | 0.18s  |
| POKER (H=50)                                                |     56.21 |       25.6788 |                             9.6913 | 0.09s  |
| ϵ-Greedy (ϵ=0.050)                                          |     65.45 |       27.3929 |                             4.0210 | 0.02s  |
| ϵ-Exploring Thompson Sampling                               |     62.82 |       27.9018 |                             9.2377 | 0.04s  |
| UCBT                                                        |     65.40 |       28.7984 |                             4.0759 | 0.03s  |
| Thompson Sampling                                           |     66.16 |       28.8956 |                             7.1444 | 0.13s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     65.94 |       29.0318 |                             7.1008 | 0.17s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     65.61 |       29.3229 |                             7.0179 | 0.17s  |
| KL-UCB                                                      |     66.78 |       29.6304 |                             7.3837 | 1.41s  |
| ReBoot (r=0.25)                                             |     61.18 |       30.3599 |                             5.2731 | 0.06s  |
| CODE (δ=0.900)                                              |     54.94 |       30.6423 |                             6.5536 | 0.08s  |
| POKER (H=100)                                               |     51.57 |       30.8991 |                            12.6895 | 0.09s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     65.55 |       31.3306 |                             4.6232 | 0.03s  |
| UCB1-Tuned                                                  |     62.03 |       31.6747 |                             3.6906 | 0.07s  |
| Vanilla Residual Bootstrap (init=0)                         |     59.99 |       33.1442 |                             5.4073 | 0.05s  |
| Non-Parametric Thompson Sampling                            |     63.70 |       33.7962 |                             7.1820 | 0.70s  |
| ReBoot (r=0.50)                                             |     58.58 |       34.0829 |                             5.9224 | 0.05s  |
| Bounded Dirichlet Sampling                                  |     63.86 |       34.1647 |                             7.1345 | 0.36s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     57.19 |       35.0506 |                             6.7983 | 0.17s  |
| ϵ-Greedy (ϵ=0.100)                                          |     63.98 |       35.8380 |                             5.3322 | 0.02s  |
| Multiplier Bootstrap-based Exploration                      |     60.70 |       36.1612 |                             4.2418 | 1.00s  |
| Kullback-Leibler Maillard Sampling                          |     59.67 |       37.5162 |                             8.3979 | 0.11s  |
| Perturbed-History Exploration (a=1.1)                       |     56.96 |       37.8929 |                             5.6711 | 0.13s  |
| POKER (H=250)                                               |     46.27 |       38.6838 |                            15.5508 | 0.09s  |
| Garbage In, Reward Out (a=0.10)                             |     57.65 |       38.7302 |                             5.2772 | 0.14s  |
| Vanilla Residual Bootstrap (init=1)                         |     59.43 |       40.6304 |                             4.7837 | 0.05s  |
| Bootstrapped Thompson Sampling (J=500)                      |     40.59 |       41.9370 |                            21.7066 | 0.67s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     40.88 |       41.9668 |                            21.1936 | 1.30s  |
| Bootstrapped Thompson Sampling (J=100)                      |     40.77 |       42.3584 |                            21.7453 | 0.17s  |
| Bootstrapped Thompson Sampling (J=10)                       |     39.55 |       42.8224 |                            21.8677 | 0.07s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     44.13 |       44.2992 |                            10.4673 | 0.18s  |
| lil' UCB (δ=0.100)                                          |     52.19 |       44.8365 |                             5.5606 | 0.07s  |
| Tsallis-INF                                                 |     54.25 |       46.4787 |                             5.9697 | 0.22s  |
| Forced Exploration                                          |     62.89 |       46.6666 |                             6.2607 | 0.02s  |
| ReBoot (r=0.90)                                             |     52.24 |       47.2795 |                             6.7367 | 0.05s  |
| Garbage In, Reward Out (a=0.33)                             |     51.74 |       49.2706 |                             5.5459 | 0.18s  |
| Vanilla Residual Bootstrap (init=5)                         |     55.69 |       50.7442 |                             6.1208 | 0.05s  |
| ReBoot (r=1.00)                                             |     49.90 |       51.8800 |                             6.7533 | 0.05s  |
| EB-TCI                                                      |     42.82 |       55.0174 |                            15.7714 | 0.07s  |
| Perturbed-History Exploration (a=2.1)                       |     47.44 |       56.5448 |                             6.0521 | 0.17s  |
| ETC (m=10)                                                  |     47.32 |       56.6956 |                            11.0554 | 0.03s  |
| lil' UCB (δ=0.010)                                          |     44.08 |       62.1486 |                             6.5312 | 0.07s  |
| Garbage In, Reward Out (a=1.00)                             |     43.03 |       66.4802 |                             6.9482 | 0.20s  |
| Boltzmann-Gumbel Exploration                                |     43.87 |       68.9250 |                             6.5817 | 0.07s  |
| ReBoot (r=1.50)                                             |     40.44 |       72.1794 |                             8.1305 | 0.05s  |
| lil' UCB (δ=0.001)                                          |     39.18 |       73.8291 |                             8.0325 | 0.08s  |
| ETC (m=5)                                                   |     27.93 |       78.7963 |                            24.1796 | 0.03s  |
| ReBoot (r=1.70)                                             |     37.41 |       79.4522 |                             8.9230 | 0.05s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     50.82 |       81.7548 |                            11.1762 | 0.02s  |
| ETC (m=20)                                                  |     49.52 |       85.1694 |                            11.9964 | 0.03s  |
| UCB1                                                        |     34.52 |       86.8474 |                            10.2054 | 0.05s  |
| ReBoot (r=2.10)                                             |     32.31 |       92.8131 |                            10.7156 | 0.05s  |
| EXP-IX                                                      |     31.85 |       95.7317 |                            13.0954 | 0.13s  |
| ETC (m=3)                                                   |     22.30 |       98.5252 |                            27.0722 | 0.03s  |
| ETC (m=25)                                                  |     41.95 |      105.2629 |                            14.8396 | 0.03s  |
| ETC (m=2)                                                   |     20.21 |      110.5641 |                            26.8868 | 0.03s  |
| Gradient Bandit                                             |     30.32 |      110.7043 |                            17.1641 | 0.08s  |
| Gradient Bandit (with baseline)                             |     31.34 |      113.5984 |                            11.7063 | 0.08s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     35.59 |      127.2145 |                            17.7947 | 0.02s  |
| CODE (δ=0.050)                                              |     10.94 |      187.9726 |                            24.8420 | 0.09s  |
| Random                                                      |     10.01 |      204.0160 |                            30.3495 | 0.01s  |

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
| UCB-DT (γ=0.90)                                             |     43.02 |       25.6120 |                             7.2004 | 0.55s  |
| UCB-DT (γ=0.95)                                             |     43.00 |       25.6319 |                             7.1816 | 0.54s  |
| UCB-DT (γ=0.75)                                             |     43.05 |       25.6700 |                             7.2075 | 0.54s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     44.11 |       26.7250 |                             8.7506 | 1.89s  |
| TS-UCB (100 samples)                                        |     45.02 |       26.9097 |                             6.2137 | 11.31s |
| MOSS-Anytime (α=-0.85)                                      |     40.04 |       27.3181 |                             8.7262 | 0.05s  |
| MOSS-Anytime (α=-0.50)                                      |     44.05 |       27.4891 |                             5.4358 | 0.05s  |
| CODE (δ=0.990)                                              |     39.41 |       27.7728 |                            10.1499 | 0.08s  |
| TS-UCB (10 samples)                                         |     44.55 |       27.9363 |                             5.9386 | 1.18s  |
| UCB-DT (γ=1.00)                                             |     38.52 |       28.0522 |                             9.8213 | 0.54s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     38.24 |       28.1487 |                             9.6145 | 0.03s  |
| Greedy                                                      |     37.83 |       28.2076 |                             9.9996 | 0.03s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     38.32 |       28.3069 |                             9.4761 | 0.03s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     39.35 |       28.3077 |                             8.7988 | 0.03s  |
| POKER (H=1)                                                 |     37.76 |       28.3667 |                            10.1082 | 0.09s  |
| POKER (H=5)                                                 |     37.76 |       28.3800 |                            10.0953 | 0.09s  |
| POKER (H=10)                                                |     37.74 |       28.4050 |                            10.0473 | 0.09s  |
| ϵ-Greedy (ϵ=0.010)                                          |     38.03 |       28.4793 |                             9.7905 | 0.03s  |
| ϵ-Greedy (ϵ=0.020)                                          |     38.36 |       28.6900 |                             9.4808 | 0.02s  |
| POKER (H=25)                                                |     37.49 |       28.8412 |                             9.4550 | 0.09s  |
| ϵ-Greedy (ϵ=0.050)                                          |     39.46 |       29.3486 |                             8.7084 | 0.02s  |
| Bootstrapped Thompson Sampling (J=10)                       |     38.57 |       29.4073 |                            13.9756 | 0.07s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     40.91 |       29.4333 |                             7.5048 | 0.03s  |
| MOSS-Anytime (α=-0.33)                                      |     42.29 |       29.8866 |                             5.9957 | 0.05s  |
| POKER (H=100)                                               |     38.92 |       29.9131 |                             6.5647 | 0.09s  |
| POKER (H=50)                                                |     36.98 |       30.4262 |                             8.3416 | 0.09s  |
| ϵ-Exploring Thompson Sampling                               |     40.14 |       30.7659 |                             8.9988 | 0.04s  |
| Bootstrapped Thompson Sampling (J=500)                      |     38.36 |       30.8943 |                            13.6813 | 0.68s  |
| Bootstrapped Thompson Sampling (J=100)                      |     38.23 |       30.9704 |                            13.6387 | 0.18s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     37.93 |       31.2238 |                            13.7505 | 1.32s  |
| ϵ-Greedy (ϵ=0.100)                                          |     40.16 |       31.5381 |                             7.6639 | 0.02s  |
| TS-UCB (1 samples)                                          |     41.21 |       31.8313 |                             6.2230 | 0.17s  |
| UCBT                                                        |     41.92 |       32.0754 |                             5.3843 | 0.03s  |
| Forced Exploration                                          |     41.72 |       33.1699 |                             5.7046 | 0.03s  |
| POKER (H=250)                                               |     37.22 |       33.9079 |                             8.0820 | 0.09s  |
| WR-SDA                                                      |     37.74 |       34.3702 |                             7.8470 | 0.56s  |
| CODE (δ=0.900)                                              |     35.87 |       35.7202 |                            11.4984 | 0.08s  |
| UCB1-Tuned                                                  |     38.36 |       36.0304 |                             5.8517 | 0.07s  |
| ReBoot (r=0.25)                                             |     35.81 |       36.8892 |                             8.1828 | 0.05s  |
| Vanilla Residual Bootstrap (init=0)                         |     35.10 |       38.0391 |                             7.9288 | 0.05s  |
| Optimistic Thompson Sampling                                |     36.78 |       38.4207 |                             7.1289 | 0.18s  |
| Multiplier Bootstrap-based Exploration                      |     36.05 |       38.7066 |                             7.0003 | 1.02s  |
| ReBoot (r=0.50)                                             |     34.21 |       39.5480 |                             8.2009 | 0.05s  |
| ETC (m=10)                                                  |     33.45 |       40.0881 |                            11.7950 | 0.03s  |
| Thompson Sampling                                           |     35.01 |       40.5420 |                             7.5125 | 0.13s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     34.96 |       40.5786 |                             7.5540 | 0.17s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     34.87 |       40.6461 |                             7.5447 | 0.17s  |
| Garbage In, Reward Out (a=0.10)                             |     33.73 |       42.0945 |                             7.6013 | 0.18s  |
| Perturbed-History Exploration (a=1.1)                       |     33.49 |       42.3004 |                             7.7267 | 0.15s  |
| KL-UCB                                                      |     34.54 |       42.7149 |                             6.2245 | 1.53s  |
| EB-TCI                                                      |     30.56 |       42.8317 |                             9.3319 | 0.07s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     32.52 |       43.1108 |                             8.0902 | 0.18s  |
| Non-Parametric Thompson Sampling                            |     33.09 |       43.6865 |                             7.5605 | 0.72s  |
| Vanilla Residual Bootstrap (init=1)                         |     32.88 |       43.7710 |                             7.4509 | 0.05s  |
| Bounded Dirichlet Sampling                                  |     32.79 |       44.7466 |                             7.9659 | 0.43s  |
| Tsallis-INF                                                 |     32.35 |       45.6862 |                             8.4068 | 0.22s  |
| lil' UCB (δ=0.100)                                          |     31.70 |       46.4287 |                             6.7023 | 0.07s  |
| Kullback-Leibler Maillard Sampling                          |     29.69 |       47.8324 |                             8.4744 | 0.12s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     27.45 |       48.1450 |                            10.2207 | 0.18s  |
| Garbage In, Reward Out (a=0.33)                             |     30.11 |       48.1458 |                             8.0648 | 0.20s  |
| ReBoot (r=0.90)                                             |     29.34 |       48.4181 |                             8.4845 | 0.05s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     33.79 |       49.1413 |                             7.5396 | 0.02s  |
| ETC (m=5)                                                   |     21.32 |       50.0278 |                            17.6885 | 0.03s  |
| ReBoot (r=1.00)                                             |     27.89 |       50.9352 |                             8.6898 | 0.05s  |
| ETC (m=20)                                                  |     31.24 |       51.1732 |                             8.6350 | 0.03s  |
| Perturbed-History Exploration (a=2.1)                       |     27.91 |       52.2188 |                             8.4423 | 0.18s  |
| Vanilla Residual Bootstrap (init=5)                         |     28.26 |       53.2834 |                             8.4062 | 0.05s  |
| ETC (m=25)                                                  |     32.18 |       56.3820 |                             8.2546 | 0.03s  |
| lil' UCB (δ=0.010)                                          |     25.83 |       56.9410 |                             8.2814 | 0.08s  |
| Garbage In, Reward Out (a=1.00)                             |     25.12 |       57.7304 |                             9.1152 | 0.22s  |
| Boltzmann-Gumbel Exploration                                |     25.61 |       58.0539 |                             8.8928 | 0.07s  |
| ReBoot (r=1.50)                                             |     22.85 |       61.0890 |                             9.6647 | 0.05s  |
| lil' UCB (δ=0.001)                                          |     22.85 |       62.7995 |                             9.1698 | 0.08s  |
| ReBoot (r=1.70)                                             |     21.38 |       64.4112 |                            10.0761 | 0.05s  |
| UCB1                                                        |     20.42 |       68.0927 |                            10.1489 | 0.05s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     24.60 |       68.8686 |                             9.8576 | 0.02s  |
| ReBoot (r=2.10)                                             |     19.16 |       69.7726 |                            10.8419 | 0.05s  |
| ETC (m=3)                                                   |     15.41 |       69.9994 |                            18.3348 | 0.03s  |
| EXP-IX                                                      |     19.31 |       71.1934 |                            11.2877 | 0.13s  |
| Gradient Bandit                                             |     19.00 |       75.4704 |                            12.4808 | 0.08s  |
| Gradient Bandit (with baseline)                             |     18.51 |       77.0723 |                            10.7145 | 0.08s  |
| ETC (m=2)                                                   |     15.27 |       80.4676 |                            18.0151 | 0.03s  |
| Random                                                      |     10.01 |      102.0080 |                            15.1748 | 0.01s  |
| CODE (δ=0.050)                                              |     10.00 |      102.0185 |                            14.8649 | 0.09s  |

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
| POKER (H=100)                                               |     18.10 |        4.0949 |                             0.1700 | 0.09s  |
| Greedy                                                      |     17.00 |        4.1498 |                             0.1100 | 0.03s  |
| POKER (H=10)                                                |     17.00 |        4.1498 |                             0.1100 | 0.09s  |
| POKER (H=1)                                                 |     17.00 |        4.1498 |                             0.1100 | 0.09s  |
| POKER (H=25)                                                |     17.00 |        4.1498 |                             0.1100 | 0.09s  |
| POKER (H=5)                                                 |     17.00 |        4.1498 |                             0.1100 | 0.09s  |
| POKER (H=50)                                                |     17.00 |        4.1499 |                             0.1100 | 0.09s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     16.90 |        4.1552 |                             0.1000 | 0.03s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     16.80 |        4.1598 |                             0.1000 | 0.03s  |
| ϵ-Greedy (ϵ=0.010)                                          |     16.64 |        4.1682 |                             0.1000 | 0.03s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     16.29 |        4.1854 |                             0.1000 | 0.03s  |
| ϵ-Greedy (ϵ=0.020)                                          |     16.25 |        4.1873 |                             0.1000 | 0.03s  |
| ϵ-Greedy (ϵ=0.050)                                          |     15.11 |        4.2447 |                             0.0900 | 0.02s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     14.77 |        4.2614 |                             0.0800 | 0.03s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     14.05 |        4.2973 |                             0.1600 | 0.02s  |
| ϵ-Greedy (ϵ=0.100)                                          |     13.97 |        4.3014 |                             0.0800 | 0.02s  |
| ϵ-Exploring Thompson Sampling                               |     13.74 |        4.3130 |                             0.1100 | 0.04s  |
| Forced Exploration                                          |     13.53 |        4.3235 |                             0.1000 | 0.03s  |
| UCB-DT (γ=0.90)                                             |     13.27 |        4.3365 |                             0.1000 | 0.54s  |
| UCB-DT (γ=0.95)                                             |     13.27 |        4.3365 |                             0.1000 | 0.54s  |
| UCB-DT (γ=1.00)                                             |     13.19 |        4.3406 |                             0.1200 | 0.54s  |
| UCB-DT (γ=0.75)                                             |     13.05 |        4.3474 |                             0.1000 | 0.54s  |
| MOSS-Anytime (α=-0.33)                                      |     13.00 |        4.3502 |                             0.2000 | 0.05s  |
| MOSS-Anytime (α=-0.85)                                      |     12.95 |        4.3526 |                             0.1800 | 0.05s  |
| MOSS-Anytime (α=-0.50)                                      |     12.94 |        4.3532 |                             0.1700 | 0.05s  |
| POKER (H=250)                                               |     12.40 |        4.3802 |                             0.2500 | 0.09s  |
| TS-UCB (100 samples)                                        |     12.17 |        4.3915 |                             0.2500 | 11.46s |
| UCBT                                                        |     12.17 |        4.3916 |                             0.4200 | 0.03s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     11.91 |        4.4043 |                             0.1500 | 0.02s  |
| Bootstrapped Thompson Sampling (J=10)                       |     11.83 |        4.4083 |                             0.1600 | 0.07s  |
| Bootstrapped Thompson Sampling (J=500)                      |     11.80 |        4.4101 |                             0.3400 | 0.68s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     11.78 |        4.4109 |                             0.3400 | 1.34s  |
| Bootstrapped Thompson Sampling (J=100)                      |     11.76 |        4.4118 |                             0.3100 | 0.18s  |
| EB-TCI                                                      |     11.56 |        4.4218 |                             0.4400 | 0.08s  |
| WR-SDA                                                      |     11.52 |        4.4238 |                             0.3200 | 0.38s  |
| TS-UCB (10 samples)                                         |     11.46 |        4.4271 |                             0.2500 | 1.19s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     11.45 |        4.4276 |                             0.2600 | 0.76s  |
| Vanilla Residual Bootstrap (init=0)                         |     11.42 |        4.4292 |                             0.3500 | 0.05s  |
| CODE (δ=0.900)                                              |     11.39 |        4.4305 |                             0.4900 | 0.08s  |
| ReBoot (r=0.25)                                             |     11.38 |        4.4311 |                             0.3500 | 0.06s  |
| ReBoot (r=0.50)                                             |     11.34 |        4.4329 |                             0.3800 | 0.05s  |
| TS-UCB (1 samples)                                          |     11.21 |        4.4395 |                             0.2400 | 0.17s  |
| CODE (δ=0.990)                                              |     11.21 |        4.4397 |                             0.1200 | 0.08s  |
| Optimistic Thompson Sampling                                |     11.20 |        4.4399 |                             0.3000 | 0.18s  |
| Garbage In, Reward Out (a=0.10)                             |     11.16 |        4.4418 |                             0.3400 | 0.18s  |
| Non-Parametric Thompson Sampling                            |     11.16 |        4.4422 |                             0.3400 | 0.72s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     11.15 |        4.4425 |                             0.3400 | 0.17s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     11.15 |        4.4426 |                             0.3300 | 0.17s  |
| Thompson Sampling                                           |     11.14 |        4.4429 |                             0.3300 | 0.13s  |
| Perturbed-History Exploration (a=1.1)                       |     11.13 |        4.4433 |                             0.3600 | 0.16s  |
| Multiplier Bootstrap-based Exploration                      |     11.12 |        4.4439 |                             0.3100 | 1.02s  |
| Vanilla Residual Bootstrap (init=1)                         |     11.11 |        4.4443 |                             0.3500 | 0.05s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     11.09 |        4.4454 |                             0.4000 | 0.18s  |
| Garbage In, Reward Out (a=0.33)                             |     11.04 |        4.4480 |                             0.3800 | 0.20s  |
| Tsallis-INF                                                 |     11.01 |        4.4497 |                             0.2700 | 0.23s  |
| KL-UCB                                                      |     10.99 |        4.4505 |                             0.2800 | 1.54s  |
| ReBoot (r=0.90)                                             |     10.94 |        4.4528 |                             0.3800 | 0.05s  |
| Kullback-Leibler Maillard Sampling                          |     10.91 |        4.4544 |                             0.3500 | 0.12s  |
| Perturbed-History Exploration (a=2.1)                       |     10.89 |        4.4557 |                             0.3400 | 0.18s  |
| Vanilla Residual Bootstrap (init=5)                         |     10.85 |        4.4574 |                             0.2700 | 0.05s  |
| lil' UCB (δ=0.100)                                          |     10.85 |        4.4575 |                             0.2600 | 0.08s  |
| ReBoot (r=1.00)                                             |     10.84 |        4.4578 |                             0.3500 | 0.05s  |
| Bounded Dirichlet Sampling                                  |     10.83 |        4.4586 |                             0.3100 | 0.40s  |
| UCB1-Tuned                                                  |     10.74 |        4.4632 |                             0.2400 | 0.07s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     10.72 |        4.4641 |                             0.3100 | 0.19s  |
| lil' UCB (δ=0.010)                                          |     10.70 |        4.4651 |                             0.2200 | 0.08s  |
| Boltzmann-Gumbel Exploration                                |     10.67 |        4.4663 |                             0.2700 | 0.08s  |
| Garbage In, Reward Out (a=1.00)                             |     10.66 |        4.4669 |                             0.2600 | 0.22s  |
| lil' UCB (δ=0.001)                                          |     10.54 |        4.4730 |                             0.2000 | 0.08s  |
| ReBoot (r=1.50)                                             |     10.49 |        4.4756 |                             0.2100 | 0.05s  |
| ReBoot (r=1.70)                                             |     10.40 |        4.4798 |                             0.1800 | 0.05s  |
| EXP-IX                                                      |     10.34 |        4.4830 |                             0.1600 | 0.12s  |
| ReBoot (r=2.10)                                             |     10.29 |        4.4854 |                             0.1400 | 0.05s  |
| ETC (m=25)                                                  |     10.27 |        4.4863 |                             0.0000 | 0.03s  |
| Gradient Bandit                                             |     10.27 |        4.4866 |                             0.1300 | 0.08s  |
| UCB1                                                        |     10.23 |        4.4883 |                             0.1600 | 0.04s  |
| Gradient Bandit (with baseline)                             |     10.23 |        4.4887 |                             0.1100 | 0.08s  |
| ETC (m=5)                                                   |     10.11 |        4.4943 |                             0.0000 | 0.03s  |
| ETC (m=20)                                                  |     10.11 |        4.4946 |                             0.0000 | 0.03s  |
| ETC (m=2)                                                   |     10.04 |        4.4982 |                             0.4300 | 0.03s  |
| ETC (m=3)                                                   |     10.04 |        4.4982 |                             0.4300 | 0.03s  |
| Random                                                      |     10.02 |        4.4992 |                             0.0500 | 0.01s  |
| CODE (δ=0.050)                                              |     10.00 |        4.5000 |                             0.0000 | 0.09s  |
| ETC (m=10)                                                  |      9.94 |        4.5030 |                             0.0000 | 0.03s  |

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
| MOSS-Anytime (α=-0.85)                                      |     54.95 |       22.2898 |                             5.6517 | 0.06s  |
| UCB-DT (γ=0.75)                                             |     54.64 |       22.4071 |                             6.1492 | 0.51s  |
| UCB-DT (γ=0.90)                                             |     54.45 |       22.4627 |                             6.1571 | 0.51s  |
| UCB-DT (γ=0.95)                                             |     54.39 |       22.4968 |                             6.1852 | 0.51s  |
| UCB-DT (γ=1.00)                                             |     53.32 |       22.6778 |                             7.3649 | 0.50s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     56.94 |       22.9408 |                             7.1147 | 2.85s  |
| CODE (δ=0.990)                                              |     51.11 |       23.5974 |                             9.3932 | 0.09s  |
| MOSS-Anytime (α=-0.50)                                      |     56.24 |       24.1465 |                             4.0881 | 0.06s  |
| TS-UCB (100 samples)                                        |     56.12 |       24.7437 |                             4.2837 | 12.24s |
| ReBoot (r=0.25)                                             |     52.26 |       24.7586 |                             8.6759 | 0.06s  |
| MOSS-Anytime (α=-0.33)                                      |     54.80 |       26.1464 |                             4.2098 | 0.06s  |
| TS-UCB (10 samples)                                         |     54.96 |       26.2734 |                             4.2721 | 1.28s  |
| UCBT                                                        |     47.49 |       28.8558 |                             8.0049 | 0.04s  |
| ReBoot (r=0.50)                                             |     51.44 |       28.9633 |                             6.3791 | 0.06s  |
| TS-UCB (1 samples)                                          |     52.69 |       29.2908 |                             4.9082 | 0.19s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     45.66 |       30.9426 |                            10.3885 | 0.03s  |
| Bootstrapped Thompson Sampling (J=10)                       |     49.88 |       31.1623 |                             6.5576 | 0.07s  |
| Forced Exploration                                          |     48.86 |       31.4112 |                             9.0715 | 0.03s  |
| Multiplier Bootstrap-based Exploration                      |     49.17 |       32.4139 |                             6.0942 | 1.03s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     42.88 |       32.7340 |                            12.0469 | 0.03s  |
| ϵ-Exploring Thompson Sampling                               |     44.38 |       33.2239 |                            12.5400 | 0.04s  |
| ϵ-Greedy (ϵ=0.100)                                          |     44.10 |       33.2831 |                            11.8153 | 0.03s  |
| ϵ-Greedy (ϵ=0.050)                                          |     42.23 |       33.7998 |                            13.3609 | 0.03s  |
| UCB1-Tuned                                                  |     48.22 |       34.0173 |                             5.5690 | 0.07s  |
| Bootstrapped Thompson Sampling (J=100)                      |     47.17 |       34.7044 |                             6.3147 | 0.18s  |
| Garbage In, Reward Out (a=0.10)                             |     46.53 |       35.1309 |                             6.4203 | 0.15s  |
| Bootstrapped Thompson Sampling (J=500)                      |     46.87 |       35.1931 |                             6.3344 | 0.70s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     46.77 |       35.2492 |                             6.3528 | 1.36s  |
| Vanilla Residual Bootstrap (init=1)                         |     46.87 |       35.3194 |                             6.1483 | 0.05s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     40.31 |       35.3717 |                            14.8777 | 0.03s  |
| Optimistic Thompson Sampling                                |     47.14 |       35.7522 |                             6.0706 | 0.20s  |
| ϵ-Greedy (ϵ=0.020)                                          |     39.94 |       35.9324 |                            16.4079 | 0.03s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     39.46 |       36.2891 |                            16.3213 | 0.03s  |
| Vanilla Residual Bootstrap (init=0)                         |     39.16 |       36.6560 |                            16.2068 | 0.05s  |
| ETC (m=5)                                                   |     39.97 |       37.5465 |                            17.0296 | 0.03s  |
| Thompson Sampling                                           |     45.10 |       37.7381 |                             6.5241 | 0.14s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     45.09 |       37.7390 |                             6.4866 | 0.19s  |
| ϵ-Greedy (ϵ=0.010)                                          |     38.41 |       37.7394 |                            18.4671 | 0.03s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     44.99 |       37.8316 |                             6.5136 | 0.19s  |
| KL-UCB                                                      |     44.75 |       37.9754 |                             5.7666 | 1.47s  |
| ETC (m=10)                                                  |     40.33 |       38.9869 |                            13.6763 | 0.03s  |
| Non-Parametric Thompson Sampling                            |     43.90 |       39.3507 |                             6.7160 | 0.72s  |
| Greedy                                                      |     36.66 |       39.9099 |                            21.7087 | 0.03s  |
| Bounded Dirichlet Sampling                                  |     43.54 |       39.9645 |                             6.6454 | 0.44s  |
| CODE (δ=0.900)                                              |     40.61 |       40.2050 |                            13.2482 | 0.09s  |
| ReBoot (r=0.90)                                             |     42.54 |       40.7881 |                             7.1968 | 0.06s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     41.59 |       40.8984 |                             7.1496 | 0.19s  |
| WR-SDA                                                      |     36.73 |       41.0443 |                            19.6162 | 0.67s  |
| POKER (H=100)                                               |     36.39 |       41.3770 |                            22.1022 | 0.08s  |
| POKER (H=250)                                               |     36.40 |       41.3832 |                            21.9572 | 0.08s  |
| POKER (H=50)                                                |     36.31 |       41.4064 |                            22.3819 | 0.09s  |
| POKER (H=25)                                                |     36.24 |       41.4405 |                            22.6058 | 0.08s  |
| Kullback-Leibler Maillard Sampling                          |     40.83 |       41.4463 |                             7.5405 | 0.12s  |
| POKER (H=10)                                                |     36.05 |       41.5399 |                            23.0560 | 0.08s  |
| POKER (H=5)                                                 |     35.92 |       41.6486 |                            23.3861 | 0.08s  |
| POKER (H=1)                                                 |     35.72 |       41.8819 |                            23.9184 | 0.09s  |
| Perturbed-History Exploration (a=1.1)                       |     40.79 |       42.7866 |                             7.3646 | 0.17s  |
| ReBoot (r=1.00)                                             |     40.65 |       43.3432 |                             7.6618 | 0.06s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     41.58 |       44.0842 |                            11.9547 | 0.03s  |
| Garbage In, Reward Out (a=0.33)                             |     38.75 |       45.1922 |                             7.8091 | 0.19s  |
| ETC (m=3)                                                   |     33.51 |       45.7840 |                            28.1017 | 0.03s  |
| ETC (m=20)                                                  |     37.94 |       47.1134 |                            13.4466 | 0.03s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     33.63 |       48.2505 |                             9.6052 | 0.19s  |
| lil' UCB (δ=0.100)                                          |     36.34 |       48.6046 |                             7.3285 | 0.08s  |
| ETC (m=25)                                                  |     37.82 |       51.7141 |                            13.9357 | 0.03s  |
| ETC (m=2)                                                   |     29.53 |       53.1694 |                            28.6333 | 0.03s  |
| ReBoot (r=1.50)                                             |     33.20 |       53.6329 |                            10.8113 | 0.06s  |
| Perturbed-History Exploration (a=2.1)                       |     32.69 |       53.7141 |                             9.5801 | 0.20s  |
| Tsallis-INF                                                 |     32.39 |       54.7917 |                            11.3371 | 0.24s  |
| Vanilla Residual Bootstrap (init=5)                         |     31.29 |       56.3702 |                            10.3022 | 0.05s  |
| ReBoot (r=1.70)                                             |     30.99 |       56.8129 |                            12.0371 | 0.06s  |
| Garbage In, Reward Out (a=1.00)                             |     29.48 |       58.3347 |                            11.5697 | 0.23s  |
| Boltzmann-Gumbel Exploration                                |     29.89 |       58.4917 |                            11.5794 | 0.08s  |
| lil' UCB (δ=0.010)                                          |     29.25 |       58.7242 |                            11.2953 | 0.08s  |
| EB-TCI                                                      |     24.42 |       59.0388 |                            22.7179 | 0.08s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     30.85 |       61.5675 |                            16.7498 | 0.03s  |
| ReBoot (r=2.10)                                             |     27.57 |       61.9376 |                            14.1671 | 0.06s  |
| lil' UCB (δ=0.001)                                          |     25.41 |       64.5631 |                            14.7753 | 0.08s  |
| UCB1                                                        |     22.31 |       69.6096 |                            17.0817 | 0.05s  |
| Gradient Bandit                                             |     20.28 |       74.1103 |                            17.5927 | 0.09s  |
| Gradient Bandit (with baseline)                             |     19.93 |       74.7782 |                            17.8747 | 0.09s  |
| EXP-IX                                                      |     17.65 |       77.6186 |                            20.0015 | 0.13s  |
| Random                                                      |      9.99 |       93.1436 |                            26.0904 | 0.01s  |
| CODE (δ=0.050)                                              |     10.00 |       93.1468 |                            25.9588 | 0.10s  |

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
| TS-UCB (100 samples)                                        |     57.85 |        6.9470 |                             2.3223 | 11.64s |
| TS-UCB (10 samples)                                         |     57.75 |        7.2685 |                             2.1360 | 1.22s  |
| POKER (H=5)                                                 |     55.21 |        7.8042 |                             1.8429 | 0.10s  |
| TS-UCB (1 samples)                                          |     57.78 |        7.8597 |                             1.9439 | 0.17s  |
| POKER (H=1)                                                 |     56.13 |        8.0894 |                             1.7428 | 0.10s  |
| POKER (H=10)                                                |     48.12 |        8.3258 |                             2.2912 | 0.09s  |
| UCB-DT (γ=1.00)                                             |     55.66 |        8.4709 |                             1.4837 | 0.55s  |
| UCB-DT (γ=0.90)                                             |     55.84 |        8.5612 |                             1.5143 | 0.54s  |
| UCB-DT (γ=0.95)                                             |     55.76 |        8.5784 |                             1.5121 | 0.55s  |
| Greedy                                                      |     53.82 |        8.6471 |                             1.5273 | 0.03s  |
| UCB-DT (γ=0.75)                                             |     55.91 |        8.6739 |                             1.5558 | 0.55s  |
| ϵ-Decreasing (ϵ=0.990)                                      |     53.57 |        8.8897 |                             1.5610 | 0.03s  |
| ϵ-Decreasing (ϵ=0.900)                                      |     53.45 |        9.0268 |                             1.5815 | 0.03s  |
| ϵ-Greedy (ϵ=0.010)                                          |     53.34 |        9.0561 |                             1.5947 | 0.03s  |
| Optimistic Thompson Sampling                                |     54.95 |        9.3835 |                             3.1222 | 0.18s  |
| ϵ-Greedy (ϵ=0.020)                                          |     52.84 |        9.4814 |                             1.6900 | 0.03s  |
| CODE (δ=0.990)                                              |     48.81 |        9.4822 |                             1.7486 | 0.09s  |
| ϵ-Decreasing (ϵ=0.700)                                      |     52.78 |        9.6599 |                             1.7204 | 0.03s  |
| WR-SDA                                                      |     52.23 |       10.2123 |                             2.7845 | 0.21s  |
| POKER (H=50)                                                |     41.34 |       10.6033 |                             2.7010 | 0.09s  |
| POKER (H=25)                                                |     41.02 |       10.6190 |                             2.7176 | 0.09s  |
| POKER (H=100)                                               |     41.56 |       10.6923 |                             2.7973 | 0.09s  |
| ϵ-Greedy (ϵ=0.050)                                          |     51.57 |       10.7107 |                             1.9386 | 0.03s  |
| POKER (H=250)                                               |     42.10 |       10.8128 |                             2.8381 | 0.09s  |
| ϵ-Exploring Thompson Sampling                               |     45.15 |       10.8883 |                             4.0500 | 0.04s  |
| MOSS-Anytime (α=-0.85)                                      |     50.92 |       11.2098 |                             1.9147 | 0.06s  |
| ϵ-Decreasing (ϵ=0.500)                                      |     50.84 |       11.5088 |                             2.0720 | 0.03s  |
| KL-UCB                                                      |     51.49 |       11.6751 |                             3.5785 | 1.24s  |
| Thompson Sampling                                           |     48.51 |       12.4396 |                             2.7769 | 0.14s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     37.00 |       12.6026 |                             4.1618 | 2.48s  |
| Satisficing Thompson Sampling (ϵ=0.005)                     |     47.58 |       12.6638 |                             2.7907 | 0.18s  |
| ϵ-Greedy (ϵ=0.100)                                          |     49.34 |       12.7974 |                             2.3308 | 0.03s  |
| Satisficing Thompson Sampling (ϵ=0.010)                     |     45.49 |       13.1710 |                             2.8249 | 0.18s  |
| Non-Parametric Thompson Sampling                            |     47.46 |       13.6038 |                             4.3455 | 0.73s  |
| Forced Exploration                                          |     48.30 |       13.9900 |                             2.5181 | 0.03s  |
| Bounded Dirichlet Sampling                                  |     45.58 |       14.5418 |                             4.6561 | 0.38s  |
| Kullback-Leibler Maillard Sampling                          |     43.53 |       15.1294 |                             5.1731 | 0.11s  |
| MOSS-Anytime (α=-0.50)                                      |     44.06 |       15.3933 |                             2.1697 | 0.06s  |
| MOSS-Anytime (α=-0.33)                                      |     40.99 |       17.0540 |                             2.2724 | 0.06s  |
| UCBT                                                        |     32.33 |       18.1863 |                             6.0728 | 0.04s  |
| Satisficing Thompson Sampling (ϵ=0.050)                     |     23.97 |       18.9613 |                             5.2597 | 0.18s  |
| EB-TCI                                                      |     35.93 |       19.7395 |                             5.2415 | 0.07s  |
| ReBoot (r=0.25)                                             |     34.89 |       19.9697 |                             3.1894 | 0.06s  |
| Vanilla Residual Bootstrap (init=0)                         |     33.47 |       21.5207 |                             3.2212 | 0.05s  |
| Multiplier Bootstrap-based Exploration                      |     28.45 |       22.2710 |                             3.5416 | 1.02s  |
| ETC (m=20)                                                  |     33.55 |       22.3233 |                             4.2529 | 0.03s  |
| ETC (m=10)                                                  |     27.09 |       22.3539 |                             6.4168 | 0.03s  |
| ReBoot (r=0.50)                                             |     30.87 |       22.5161 |                             3.8147 | 0.06s  |
| ϵ-Decreasing (ϵ=0.200)                                      |     38.74 |       22.6530 |                             4.3599 | 0.03s  |
| UCB1-Tuned                                                  |     25.07 |       22.9077 |                             3.4824 | 0.07s  |
| Vanilla Residual Bootstrap (init=1)                         |     31.19 |       23.1578 |                             3.3412 | 0.05s  |
| Tsallis-INF                                                 |     26.30 |       23.2635 |                             4.3108 | 0.22s  |
| Vanilla Residual Bootstrap (init=5)                         |     30.48 |       23.8825 |                             3.5154 | 0.05s  |
| Garbage In, Reward Out (a=0.10)                             |     26.82 |       23.9510 |                             3.8778 | 0.15s  |
| Perturbed-History Exploration (a=1.1)                       |     24.17 |       24.8624 |                             4.3134 | 0.17s  |
| Satisficing Thompson Sampling (ϵ=0.100)                     |     12.50 |       26.5896 |                             8.8139 | 0.18s  |
| ETC (m=25)                                                  |     28.64 |       27.0247 |                             5.2417 | 0.03s  |
| CODE (δ=0.900)                                              |     16.26 |       27.7259 |                             4.4425 | 0.09s  |
| Garbage In, Reward Out (a=0.33)                             |     21.22 |       28.0093 |                             4.7583 | 0.21s  |
| Bootstrapped Thompson Sampling (J=500)                      |     17.05 |       28.0954 |                            16.5475 | 0.69s  |
| Bootstrapped Thompson Sampling (J=1000)                     |     16.11 |       28.1867 |                            16.6249 | 1.34s  |
| ReBoot (r=0.90)                                             |     24.08 |       28.2376 |                             5.0547 | 0.06s  |
| lil' UCB (δ=0.100)                                          |     19.19 |       28.5694 |                             4.7509 | 0.08s  |
| Bootstrapped Thompson Sampling (J=100)                      |     15.82 |       29.0489 |                            16.7117 | 0.18s  |
| ReBoot (r=1.00)                                             |     22.53 |       29.7884 |                             5.3791 | 0.06s  |
| Bootstrapped Thompson Sampling (J=10)                       |     15.12 |       30.0861 |                            17.7177 | 0.07s  |
| Perturbed-History Exploration (a=2.1)                       |     18.72 |       30.3983 |                             5.2058 | 0.19s  |
| lil' UCB (δ=0.010)                                          |     16.72 |       32.2288 |                             5.5208 | 0.08s  |
| Garbage In, Reward Out (a=1.00)                             |     17.26 |       32.4632 |                             5.6672 | 0.22s  |
| ϵ-Decreasing (ϵ=0.100)                                      |     27.51 |       32.6383 |                             6.3517 | 0.03s  |
| Boltzmann-Gumbel Exploration                                |     17.44 |       32.7460 |                             5.6438 | 0.08s  |
| lil' UCB (δ=0.001)                                          |     15.51 |       34.2797 |                             5.8524 | 0.08s  |
| EXP-IX                                                      |     15.63 |       34.8238 |                             6.2558 | 0.13s  |
| ReBoot (r=1.50)                                             |     18.20 |       35.2644 |                             6.5573 | 0.06s  |
| UCB1                                                        |     14.55 |       36.1248 |                             6.3580 | 0.05s  |
| ReBoot (r=1.70)                                             |     17.25 |       36.7828 |                             6.9301 | 0.06s  |
| ReBoot (r=2.10)                                             |     15.90 |       39.2124 |                             7.5247 | 0.06s  |
| Gradient Bandit                                             |     13.72 |       39.5229 |                             8.1141 | 0.09s  |
| Gradient Bandit (with baseline)                             |     13.15 |       40.8926 |                             7.4944 | 0.09s  |
| ETC (m=5)                                                   |     12.36 |       41.7571 |                             9.1900 | 0.03s  |
| ETC (m=3)                                                   |     12.03 |       43.5920 |                             9.6906 | 0.03s  |
| ETC (m=2)                                                   |     11.03 |       45.2564 |                             9.3287 | 0.04s  |
| CODE (δ=0.050)                                              |     10.00 |       49.2639 |                             9.8811 | 0.10s  |
| Random                                                      |      9.99 |       49.2870 |                            10.0029 | 0.01s  |

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
