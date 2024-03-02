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
| Algorithm                                                   | Regret (Mean) | Regret (Median Absolute Deviation) |  Time   |
| ----------------------------------------------------------- | ------------: | ---------------------------------: | :-----: |
| TS-UCB                                                      |       17.7488 |                             3.5854 | 0.9995s |
| Greedy                                                      |       19.6956 |                             2.4935 | 0.0195s |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |       20.8493 |                             6.6689 | 3.5360s |
| WR-SDA                                                      |       23.6906 |                             4.9393 | 0.3481s |
| Multiplier Bootstrap-based Exploration                      |       25.6195 |                             3.5732 | 1.0230s |
| Thompson Sampling                                           |       28.5425 |                             7.3473 | 0.1284s |
| KL-UCB                                                      |       29.6736 |                             7.4188 | 1.4303s |
| ϵ-Exploring Thompson Sampling                               |       29.6957 |                             9.2274 | 0.0308s |
| UCB1-Tuned                                                  |       31.6907 |                             3.7342 | 0.0501s |
| Non-Parametric Thompson Sampling                            |       33.6272 |                             7.0889 | 0.9276s |
| Bounded Dirichlet Sampling                                  |       34.3370 |                             7.2838 | 0.4493s |
| Perturbed History Exploration                               |       37.8449 |                             5.8309 | 0.1306s |
| Kullback-Leibler Maillard Sampling                          |       37.9779 |                             8.4980 | 0.1076s |
| EB-TCI                                                      |       55.9930 |                            16.8330 | 0.0680s |
| Boltzmann-Gumbel Exploration                                |       67.6782 |                             6.4929 | 0.0706s |
| UCB1                                                        |       87.2114 |                            10.3016 | 0.0404s |
| Gradient Bandit (with baseline)                             |      115.2261 |                            12.8419 | 0.0715s |
| Gradient Bandit                                             |      116.1128 |                            17.7241 | 0.0702s |
| Random                                                      |      204.1889 |                            30.4253 | 0.0044s |
<!-- END mdsh -->
