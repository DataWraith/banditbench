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
| Algorithm                                                   | Mean Regret | Regret Median Absolute Deviation |   Time   |
| ----------------------------------------------------------- | ----------: | -------------------------------: | :------: |
| TS-UCB                                                      |     15.2798 |                           2.8848 | 6.5491s  |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |     20.3089 |                           5.9755 | 30.0906s |
| WR-SDA                                                      |     21.2868 |                           4.5954 | 1.5031s  |
| Thompson Sampling                                           |     24.0018 |                           7.1198 | 0.7162s  |
| KL-UCB                                                      |     25.4425 |                           6.8216 | 7.4311s  |
| ϵ-Exploring Thompson Sampling                               |     26.0798 |                           9.4486 | 0.1902s  |
| Multiplier Bootstrap-based Exploration                      |     27.0532 |                           4.0394 | 6.3964s  |
| Greedy                                                      |     27.4696 |                           3.1817 | 0.1210s  |
| UCB1-Tuned                                                  |     28.8584 |                           4.2908 | 0.1820s  |
| Non-Parametric Thompson Sampling                            |     28.9590 |                           7.1298 | 5.1247s  |
| Bounded Dirichlet Sampling                                  |     29.6570 |                           7.1847 | 2.2065s  |
| Kullback-Leibler Maillard Sampling                          |     32.5751 |                           8.4474 | 0.6022s  |
| EB-TCI                                                      |     59.5991 |                          20.4111 | 0.3846s  |
| Boltzmann-Gumbel Exploration                                |     70.9670 |                           7.0935 | 0.4083s  |
| UCB1                                                        |     87.9779 |                          11.0318 | 0.1587s  |
| Gradient Bandit (with baseline)                             |    115.8236 |                          11.9148 | 0.4403s  |
| Gradient Bandit                                             |    116.3756 |                          17.5131 | 0.4335s  |
| Random                                                      |    204.1889 |                          30.4253 | 0.0187s  |
<!-- END mdsh -->
