#!/bin/bash
#MISE description="Run all bandit algorithms on all experiments"

declare -a EXPERIMENTS=(
  "uniform"
  "half_range"
  "hard"
  "beta"
  "reverse_beta"
)

for EXPERIMENT in "${EXPERIMENTS[@]}";
do
  RESULT_FILE="$EXPERIMENT.csv"
  REPORT_FILE="$EXPERIMENT.md"

  rm -f $RESULT_FILE

  bkt --modtime=./src/bandits/ucb/bayes_ucb.rs -- cargo run --release -- $EXPERIMENT bayesucb >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT bds >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT bge >> $RESULT_FILE
  bkt --modtime=./src/bandits/gittins/brezzi_and_lai_approximation.rs -- cargo run --release -- $EXPERIMENT brezzilai >> $RESULT_FILE
  bkt --modtime=./src/bandits/bootstrap/bts.rs -- cargo run --release -- $EXPERIMENT bts >> $RESULT_FILE
  bkt --modtime=./src/bandits/code.rs -- cargo run --release -- $EXPERIMENT code >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT ebtci >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT epsts >> $RESULT_FILE
  bkt --modtime=./src/bandits/baselines/eps_decreasing.rs -- cargo run --release -- $EXPERIMENT epsdecreasing >> $RESULT_FILE
  bkt --modtime=./src/bandits/baselines/eps_greedy.rs -- cargo run --release -- $EXPERIMENT epsgreedy >> $RESULT_FILE
  bkt --modtime=./src/bandits/baselines/eps_tsucb.rs -- cargo run --release -- $EXPERIMENT epstsucb >> $RESULT_FILE
  bkt --modtime=./src/bandits/baselines/etc.rs -- cargo run --release -- $EXPERIMENT etc >> $RESULT_FILE
  bkt --modtime=./src/bandits/exp_ix.rs -- cargo run --release -- $EXPERIMENT exp_ix >> $RESULT_FILE
  bkt --modtime=./src/bandits/forced_exploration.rs -- cargo run --release -- $EXPERIMENT fe >> $RESULT_FILE
  bkt --modtime=./src/bandits/bootstrap/giro.rs -- cargo run --release -- $EXPERIMENT giro >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT gradient >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT greedy >> $RESULT_FILE
  bkt --modtime=./src/bandits/ucb/hellinger_ucb.rs -- cargo run --release -- $EXPERIMENT hellinger >> $RESULT_FILE
  bkt --modtime=./src/bandits/ts/irs_fh.rs -- cargo run --release -- $EXPERIMENT irsfh >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT klms >> $RESULT_FILE
  bkt --modtime=./src/bandits/ucb/kl_ucb.rs -- cargo run --release -- $EXPERIMENT klucb >> $RESULT_FILE
  bkt --modtime=./src/bandits/baselines/least_failures.rs -- cargo run --release -- $EXPERIMENT lf >> $RESULT_FILE
  bkt --modtime=./src/bandits/ucb/lilucb.rs -- cargo run --release -- $EXPERIMENT lilucb >> $RESULT_FILE
  bkt --modtime=./src/bandits/bootstrap/mars.rs -- cargo run --release -- $EXPERIMENT mars >> $RESULT_FILE
  bkt --modtime=./src/bandits/bootstrap/mbe.rs -- cargo run --release -- $EXPERIMENT mbe >> $RESULT_FILE
  bkt --modtime=./src/bandits/ucb/moss_anytime.rs -- cargo run --release -- $EXPERIMENT mossanytime >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT npts >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT ots >> $RESULT_FILE
  bkt --modtime=./src/bandits/automata/pfla.rs -- cargo run --release -- $EXPERIMENT pfla >> $RESULT_FILE
  bkt --modtime=./src/bandits/bootstrap/phe.rs -- cargo run --release -- $EXPERIMENT phe >> $RESULT_FILE
  bkt --modtime=./src/bandits/poker.rs -- cargo run --release -- $EXPERIMENT poker >> $RESULT_FILE
  bkt --modtime=./src/bandits/bootstrap/reboot.rs -- cargo run --release -- $EXPERIMENT reboot >> $RESULT_FILE
  bkt --modtime=./src/bandits/ucb/reucb.rs -- cargo run --release -- $EXPERIMENT reucb >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT random >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT sts >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT ts >> $RESULT_FILE
  bkt --modtime=./src/bandits/tsucb.rs -- cargo run --release -- $EXPERIMENT tsucb >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT tsvha >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT tsallisinf >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT ucb1 >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT ucb1tuned >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT ucbdt >> $RESULT_FILE
  bkt --modtime=./src/bandits/ucb/ucbt.rs -- cargo run --release -- $EXPERIMENT ucbt >> $RESULT_FILE
  bkt --modtime=./src/bandits/bootstrap/vresboot.rs -- cargo run --release -- $EXPERIMENT vresboot >> $RESULT_FILE
  bkt --modtime=./src/bandits/gittins/whittle_approximation.rs -- cargo run --release -- $EXPERIMENT wa >> $RESULT_FILE
  bkt --modtime=./src/bandits/bootstrap/weighted_bootstrap.rs -- cargo run --release -- $EXPERIMENT wb >> $RESULT_FILE
  bkt -- cargo run --release -- $EXPERIMENT wrsda >> $RESULT_FILE

  sort -n -t ';' -k 3,3 $RESULT_FILE -o $RESULT_FILE

  echo "| Algorithm | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) | Time |" > $REPORT_FILE
  echo "|---|---:|---:|---:|:--:|" >> $REPORT_FILE

  cat $RESULT_FILE | sed 's/.*/|&|/' | sed 's/\;/\|/g' >> $REPORT_FILE
done
