# Bandit Bench

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

This project is a small, unscientific benchmark of algorithms for the Bernoulli
Multi-Armed Bandit. It benchmarks my specific use-case of short-horizon problems
(500 arm pulls) with Bernoulli rewards (i.e., either there is a reward or there
is not with a given probability).

Algorithms are only included in the benchmark if

- They are easy to implement
- They do not depend on the time horizon explicitly
- They do not need (much) parameter tuning

## Experiment

The experiment runs the MAB algorithms on several thousand Bernoulli bandit
instances and tallies up their average regret. The reward means for each arm are selected
uniformly from the \[0, 1\] interval for now.

## Algorithms

- Random Baseline (chooses arms randomly)
- Greedy Baseline (chooses the arm with the maximum average reward)
- [Boltzmann-Gumbel Exploration](https://arxiv.org/abs/1705.10257)
- [Bounded Dirichlet Sampling](https://arxiv.org/abs/2111.09724)
- [EB-TCI](https://arxiv.org/abs/2206.05979)
- [ϵ-Exploring Thompson Sampling](https://proceedings.mlr.press/v202/jin23b/jin23b.pdf) (PDF)
- [Gradient Bandit](https://arxiv.org/abs/2402.17235)
- [KL-UCB](https://arxiv.org/abs/1102.2490)
- [Kullback-Leibler Maillard Sampling](https://arxiv.org/abs/2304.14989)
- [Multiplier Bootstrap-based Exploration](https://arxiv.org/abs/2302.01543)
- [Non-Parametric Thompson Sampling](https://proceedings.mlr.press/v117/riou20a.html)
- [Thompson Sampling with Virtual Helping Agents (C3)](https://arxiv.org/abs/2209.08197)
- Thompson Sampling
- [TS-UCB](https://arxiv.org/abs/2006.06372)
- [UCB1](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [UCB1-Tuned](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [WR-SDA](https://arxiv.org/abs/2010.14323)

## Results

<!-- `> cargo run --release` -->
<!-- BEGIN mdsh -->
| Algorithm                                                   | Regret (Mean) | Regret (Median Absolute Deviation) | Time  |
| ----------------------------------------------------------- | ------------: | ---------------------------------: | :---: |
| TS-UCB                                                      |       17.7488 |                             3.5854 | 1.27s |
| Greedy                                                      |       19.6956 |                             2.4935 | 0.02s |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |       20.8493 |                             6.6689 | 3.66s |
| WR-SDA                                                      |       23.6906 |                             4.9393 | 0.36s |
| Multiplier Bootstrap-based Exploration                      |       25.6195 |                             3.5732 | 1.07s |
| Thompson Sampling                                           |       28.5425 |                             7.3473 | 0.13s |
| KL-UCB                                                      |       29.6736 |                             7.4188 | 1.49s |
| ϵ-Exploring Thompson Sampling                               |       29.6957 |                             9.2274 | 0.03s |
| UCB1-Tuned                                                  |       31.6907 |                             3.7342 | 0.05s |
| Non-Parametric Thompson Sampling                            |       33.6272 |                             7.0889 | 0.96s |
| Bounded Dirichlet Sampling                                  |       34.3370 |                             7.2838 | 0.47s |
| Perturbed History Exploration                               |       37.8449 |                             5.8309 | 0.13s |
| Kullback-Leibler Maillard Sampling                          |       37.9779 |                             8.4980 | 0.11s |
| Garbage In Reward Out                                       |       44.7215 |                             4.8564 | 0.15s |
| EB-TCI                                                      |       55.9930 |                            16.8330 | 0.07s |
| Boltzmann-Gumbel Exploration                                |       67.6782 |                             6.4929 | 0.07s |
| UCB1                                                        |       87.2114 |                            10.3016 | 0.04s |
| Gradient Bandit (with baseline)                             |      115.2261 |                            12.8419 | 0.07s |
| Gradient Bandit                                             |      116.1128 |                            17.7241 | 0.07s |
| Random                                                      |      204.1889 |                            30.4253 | 0.01s |
<!-- END mdsh -->
