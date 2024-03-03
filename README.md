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
- [Garbage In, Reward Out](http://proceedings.mlr.press/v97/kveton19a/kveton19a.pdf) (PDF)
- [Gradient Bandit](https://arxiv.org/abs/2402.17235)
- [KL-UCB](https://arxiv.org/abs/1102.2490)
- [Kullback-Leibler Maillard Sampling](https://arxiv.org/abs/2304.14989)
- [Multiplier Bootstrap-based Exploration](https://arxiv.org/abs/2302.01543)
- [Non-Parametric Thompson Sampling](https://proceedings.mlr.press/v117/riou20a.html)
- [Perturbed-History Exploration](https://arxiv.org/abs/1902.10089)
- [ReBoot](https://arxiv.org/abs/2002.08436)
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
| TS-UCB                                                      |    72.88% |       17.8546 |                             3.5976 | 5.59s  |
| ReBoot                                                      |    70.63% |       18.3477 |                             2.5414 | 0.15s  |
| Greedy                                                      |    67.48% |       19.7483 |                             2.4973 | 0.11s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    63.36% |       21.1298 |                             6.2710 | 23.79s |
| WR-SDA                                                      |    67.66% |       23.8199 |                             5.0460 | 1.58s  |
| Multiplier Bootstrap-based Exploration                      |    67.82% |       26.0614 |                             3.6393 | 5.81s  |
| ϵ-Exploring Thompson Sampling                               |    64.31% |       27.5471 |                             8.9868 | 0.16s  |
| Thompson Sampling                                           |    67.00% |       28.9445 |                             7.1632 | 0.61s  |
| KL-UCB                                                      |    67.56% |       29.6893 |                             7.4957 | 7.40s  |
| UCB1-Tuned                                                  |    62.81% |       31.7769 |                             3.6345 | 0.26s  |
| Non-Parametric Thompson Sampling                            |    64.59% |       33.8504 |                             7.0679 | 5.27s  |
| Bounded Dirichlet Sampling                                  |    64.70% |       34.2376 |                             7.1518 | 2.51s  |
| Kullback-Leibler Maillard Sampling                          |    60.53% |       37.5467 |                             8.4138 | 0.51s  |
| Perturbed-History Exploration                               |    57.78% |       37.8970 |                             5.6488 | 0.71s  |
| Garbage In, Reward Out                                      |    57.08% |       44.4496 |                             4.8697 | 0.80s  |
| EB-TCI                                                      |    42.95% |       56.0202 |                            16.1098 | 0.33s  |
| Boltzmann-Gumbel Exploration                                |    44.52% |       69.1820 |                             6.7076 | 0.35s  |
| UCB1                                                        |    34.84% |       87.3965 |                            10.1205 | 0.16s  |
| Gradient Bandit                                             |    30.56% |      111.1047 |                            17.4381 | 0.41s  |
| Gradient Bandit (with baseline)                             |    31.78% |      114.0673 |                            11.6366 | 0.42s  |
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
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    44.83% |       26.7704 |                             8.7872 | 11.29s |
| ReBoot                                                      |    39.95% |       27.7705 |                             9.3591 | 0.16s  |
| Greedy                                                      |    39.00% |       28.0151 |                             9.7636 | 0.11s  |
| TS-UCB                                                      |    45.12% |       28.1337 |                             6.0061 | 5.76s  |
| ϵ-Exploring Thompson Sampling                               |    41.08% |       30.8109 |                             9.0357 | 0.17s  |
| Multiplier Bootstrap-based Exploration                      |    42.47% |       30.9818 |                             6.6402 | 5.89s  |
| WR-SDA                                                      |    38.17% |       34.3574 |                             7.8687 | 2.54s  |
| UCB1-Tuned                                                  |    39.23% |       36.0362 |                             5.7070 | 0.27s  |
| Thompson Sampling                                           |    35.68% |       40.6934 |                             7.4756 | 0.63s  |
| Perturbed-History Exploration                               |    34.15% |       42.4480 |                             7.6337 | 0.80s  |
| KL-UCB                                                      |    35.22% |       42.8549 |                             6.2878 | 7.77s  |
| EB-TCI                                                      |    30.68% |       43.1680 |                             8.8295 | 0.36s  |
| Non-Parametric Thompson Sampling                            |    33.66% |       43.8953 |                             7.4578 | 5.44s  |
| Bounded Dirichlet Sampling                                  |    33.37% |       44.9539 |                             7.9732 | 3.16s  |
| Garbage In, Reward Out                                      |    32.82% |       44.9909 |                             7.5012 | 0.99s  |
| Kullback-Leibler Maillard Sampling                          |    30.15% |       48.1212 |                             8.2677 | 0.53s  |
| Boltzmann-Gumbel Exploration                                |    25.93% |       58.3994 |                             8.7698 | 0.35s  |
| UCB1                                                        |    20.65% |       68.4993 |                            10.1090 | 0.16s  |
| Gradient Bandit                                             |    19.16% |       75.6775 |                            12.1688 | 0.41s  |
| Gradient Bandit (with baseline)                             |    18.70% |       77.4743 |                            10.5750 | 0.43s  |
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
| Algorithm                                                   | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) | Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :---: |
| Greedy                                                      |    16.72% |        4.1640 |                             0.1100 | 0.12s |
| ReBoot                                                      |    14.03% |        4.2983 |                             0.1100 | 0.17s |
| ϵ-Exploring Thompson Sampling                               |    13.51% |        4.3245 |                             0.1100 | 0.19s |
| EB-TCI                                                      |    11.55% |        4.4225 |                             0.4400 | 0.40s |
| TS-UCB                                                      |    11.55% |        4.4227 |                             0.2400 | 6.29s |
| Multiplier Bootstrap-based Exploration                      |    11.47% |        4.4263 |                             0.2500 | 6.13s |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    11.45% |        4.4273 |                             0.2600 | 4.57s |
| WR-SDA                                                      |    11.45% |        4.4275 |                             0.3200 | 1.81s |
| Non-Parametric Thompson Sampling                            |    11.16% |        4.4418 |                             0.4000 | 5.32s |
| Perturbed-History Exploration                               |    11.15% |        4.4425 |                             0.4200 | 1.01s |
| Garbage In, Reward Out                                      |    11.15% |        4.4426 |                             0.4100 | 1.00s |
| Thompson Sampling                                           |    11.15% |        4.4427 |                             0.4200 | 0.75s |
| KL-UCB                                                      |    11.02% |        4.4490 |                             0.2300 | 8.03s |
| Kullback-Leibler Maillard Sampling                          |    10.93% |        4.4533 |                             0.3400 | 0.60s |
| Bounded Dirichlet Sampling                                  |    10.86% |        4.4572 |                             0.2900 | 2.87s |
| UCB1-Tuned                                                  |    10.76% |        4.4620 |                             0.4400 | 0.28s |
| Boltzmann-Gumbel Exploration                                |    10.68% |        4.4660 |                             0.2600 | 0.39s |
| UCB1                                                        |    10.24% |        4.4880 |                             0.1600 | 0.16s |
| Gradient Bandit (with baseline)                             |    10.20% |        4.4899 |                             0.1100 | 0.42s |
| Gradient Bandit                                             |    10.18% |        4.4908 |                             0.1300 | 0.41s |
| Random                                                      |     9.98% |        4.5009 |                             0.0500 | 0.03s |
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
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    56.91% |       23.2902 |                             7.1493 | 17.59s |
| Multiplier Bootstrap-based Exploration                      |    54.92% |       25.7531 |                             5.7460 | 5.74s  |
| TS-UCB                                                      |    54.99% |       26.7554 |                             4.4802 | 5.95s  |
| ϵ-Exploring Thompson Sampling                               |    44.70% |       33.6912 |                            12.4300 | 0.17s  |
| UCB1-Tuned                                                  |    48.78% |       34.1720 |                             5.7265 | 0.28s  |
| Garbage In, Reward Out                                      |    46.27% |       36.5880 |                             6.6192 | 0.82s  |
| Thompson Sampling                                           |    45.50% |       38.0338 |                             6.6413 | 0.64s  |
| KL-UCB                                                      |    45.13% |       38.3085 |                             5.9510 | 7.62s  |
| Non-Parametric Thompson Sampling                            |    44.28% |       39.6896 |                             6.8661 | 4.20s  |
| Greedy                                                      |    37.36% |       39.9645 |                            20.3130 | 0.13s  |
| Bounded Dirichlet Sampling                                  |    44.03% |       40.2371 |                             6.7909 | 2.46s  |
| WR-SDA                                                      |    37.82% |       40.8505 |                            18.3470 | 2.84s  |
| ReBoot                                                      |    37.09% |       41.4817 |                            21.4545 | 0.15s  |
| Kullback-Leibler Maillard Sampling                          |    41.32% |       41.7427 |                             7.4157 | 0.51s  |
| Perturbed-History Exploration                               |    41.26% |       43.0633 |                             7.6161 | 0.88s  |
| EB-TCI                                                      |    24.85% |       58.9761 |                            22.9968 | 0.33s  |
| Boltzmann-Gumbel Exploration                                |    30.21% |       59.0762 |                            11.4529 | 0.36s  |
| UCB1                                                        |    22.44% |       70.4627 |                            16.8609 | 0.17s  |
| Gradient Bandit                                             |    20.43% |       75.0125 |                            17.3070 | 0.40s  |
| Gradient Bandit (with baseline)                             |    20.06% |       75.7085 |                            17.5892 | 0.42s  |
| Random                                                      |     9.99% |       94.2791 |                            25.9206 | 0.04s  |
<!-- END mdsh -->

</details>

## Remarks

* Greedy does surprisingly well (on average) over short time horizons, and the algorithms that have greedy components
  (e.g., EB-TCI, ϵ-Exploring Thompson Sampling) do well on the `hard` instances.
* ReBoot is very fast, given its performance. It does badly on the `beta` experiment though, not sure why.
* TS-UCB appears to be best overall; TS-VHA is close, but much slower and somewhat less consistent (higher MAD)
* WR-SDA does better than BDS, which is interesting because it is a predecessor of BDS
* My KL-UCB implementation does relatively poorly, which is a bit strange. Maybe a bug I missed?
