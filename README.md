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
- [Garbage In, Reward Out](http://proceedings.mlr.press/v97/kveton19a/kveton19a.pdf) (PDF)
- [Gradient Bandit](https://arxiv.org/abs/2402.17235)
- [KL-UCB](https://arxiv.org/abs/1102.2490)
- [Kullback-Leibler Maillard Sampling](https://arxiv.org/abs/2304.14989)
- [Multiplier Bootstrap-based Exploration](https://arxiv.org/abs/2302.01543)
- [Non-Parametric Thompson Sampling](https://proceedings.mlr.press/v117/riou20a.html)
- [Perturbed-History Exploration](https://arxiv.org/abs/1902.10089)
- [Thompson Sampling with Virtual Helping Agents (C3)](https://arxiv.org/abs/2209.08197)
- Thompson Sampling
- [TS-UCB](https://arxiv.org/abs/2006.06372)
- [UCB1](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [UCB1-Tuned](https://homes.di.unimi.it/~cesabian/Pubblicazioni/ml-02.pdf) (PDF)
- [WR-SDA](https://arxiv.org/abs/2010.14323)

## Results

<!-- `> cargo run --release --bin experiment1` -->
<!-- BEGIN mdsh -->
| Algorithm                                                   | % Optimal | Regret (Mean) | Regret (Median Absolute Deviation) | Time  |
| ----------------------------------------------------------- | --------: | ------------: | ---------------------------------: | :---: |
| TS-UCB                                                      |    72.78% |       17.7544 |                             3.5920 | 1.27s |
| Greedy                                                      |    67.26% |       19.6907 |                             2.4604 | 0.02s |
| Thompson Sampling with Virtual Helping Agents (Combiner C3) |    62.95% |       20.8086 |                             6.2404 | 4.07s |
| WR-SDA                                                      |    67.33% |       24.2847 |                             5.0947 | 0.34s |
| ϵ-Exploring Thompson Sampling                               |    63.87% |       24.8700 |                             7.5262 | 0.03s |
| Multiplier Bootstrap-based Exploration                      |    67.36% |       25.8539 |                             3.6983 | 1.08s |
| Thompson Sampling                                           |    66.98% |       28.9104 |                             6.9174 | 0.13s |
| KL-UCB                                                      |    67.32% |       29.6725 |                             7.4190 | 1.48s |
| UCB1-Tuned                                                  |    62.60% |       31.6907 |                             3.7342 | 0.05s |
| Non-Parametric Thompson Sampling                            |    64.24% |       33.9677 |                             6.9821 | 0.95s |
| Bounded Dirichlet Sampling                                  |    64.41% |       34.2663 |                             7.1895 | 0.44s |
| Kullback-Leibler Maillard Sampling                          |    59.94% |       38.0329 |                             8.4795 | 0.11s |
| Perturbed-History Exploration                               |    57.27% |       38.4733 |                             5.9363 | 0.14s |
| Garbage In, Reward Out                                      |    56.56% |       44.9933 |                             4.9542 | 0.15s |
| EB-TCI                                                      |    43.98% |       54.5253 |                            15.7085 | 0.07s |
| Boltzmann-Gumbel Exploration                                |    43.41% |       71.3422 |                             6.9133 | 0.07s |
| UCB1                                                        |    34.76% |       87.2114 |                            10.3016 | 0.04s |
| Gradient Bandit                                             |    31.12% |      108.9022 |                            19.1895 | 0.07s |
| Gradient Bandit (with baseline)                             |    32.54% |      111.9578 |                            12.0261 | 0.07s |
| Random                                                      |    10.00% |      204.1340 |                            30.4089 | 0.00s |
<!-- END mdsh -->
