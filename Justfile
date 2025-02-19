run_all:
	@just run uniform
	@just run half_range
	@just run hard
	@just run beta
	@just run reverse_beta
	@just aggregate
	mdsh

run EXPERIMENT:
	rm -f {{EXPERIMENT}}.csv

	bkt --modtime=./src/bandits/ucb/bayes_ucb.rs -- cargo run --release -- {{EXPERIMENT}} bayesucb >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} bds >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} bge >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/gittins/brezzi_and_lai_approximation.rs -- cargo run --release -- {{EXPERIMENT}} brezzilai >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/bootstrap/bts.rs -- cargo run --release -- {{EXPERIMENT}} bts >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/code.rs -- cargo run --release -- {{EXPERIMENT}} code >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ebtci >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} epsts >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/baselines/eps_tsucb.rs -- cargo run --release -- {{EXPERIMENT}} epstsucb >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/baselines/eps_greedy.rs -- cargo run --release -- {{EXPERIMENT}} epsgreedy >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/baselines/eps_decreasing.rs -- cargo run --release -- {{EXPERIMENT}} epsdecreasing >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/baselines/etc.rs -- cargo run --release -- {{EXPERIMENT}} etc >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/exp_ix.rs -- cargo run --release -- {{EXPERIMENT}} exp_ix >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/forced_exploration.rs -- cargo run --release -- {{EXPERIMENT}} fe >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/bootstrap/giro.rs -- cargo run --release -- {{EXPERIMENT}} giro >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} gradient >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} greedy >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/ucb/hellinger_ucb.rs -- cargo run --release -- {{EXPERIMENT}} hellinger >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/ts/irs_fh.rs -- cargo run --release -- {{EXPERIMENT}} irsfh >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} klms >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/ucb/kl_ucb.rs -- cargo run --release -- {{EXPERIMENT}} klucb >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/baselines/least_failures.rs -- cargo run --release -- {{EXPERIMENT}} lf >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/ucb/lilucb.rs -- cargo run --release -- {{EXPERIMENT}} lilucb >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/bootstrap/mars.rs -- cargo run --release -- {{EXPERIMENT}} mars >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/bootstrap/mbe.rs -- cargo run --release -- {{EXPERIMENT}} mbe >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/ucb/moss_anytime.rs -- cargo run --release -- {{EXPERIMENT}} mossanytime >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} npts >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ots >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/automata/pfla.rs -- cargo run --release -- {{EXPERIMENT}} pfla >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/bootstrap/phe.rs -- cargo run --release -- {{EXPERIMENT}} phe >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/poker.rs -- cargo run --release -- {{EXPERIMENT}} poker >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/bootstrap/reboot.rs -- cargo run --release -- {{EXPERIMENT}} reboot >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/ucb/reucb.rs -- cargo run --release -- {{EXPERIMENT}} reucb >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} random >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} sts >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ts >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/tsucb.rs -- cargo run --release -- {{EXPERIMENT}} tsucb >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} tsvha >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} tsallisinf >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ucb1 >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ucb1tuned >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ucbdt >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/ucb/ucbt.rs -- cargo run --release -- {{EXPERIMENT}} ucbt >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/bootstrap/vresboot.rs -- cargo run --release -- {{EXPERIMENT}} vresboot >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/gittins/whittle_approximation.rs -- cargo run --release -- {{EXPERIMENT}} wa >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/bootstrap/weighted_bootstrap.rs -- cargo run --release -- {{EXPERIMENT}} wb >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} wrsda >> {{EXPERIMENT}}.csv

	sort -n -t ';' -k 3,3 {{EXPERIMENT}}.csv -o {{EXPERIMENT}}.csv

	echo "| Algorithm | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) | Time |" > {{EXPERIMENT}}.md
	echo "|---|---:|---:|---:|:--:|" >> {{EXPERIMENT}}.md

	cat {{EXPERIMENT}}.csv | sed 's/.*/|&|/' | sed 's/\;/\|/g' >> {{EXPERIMENT}}.md

aggregate:
	uv run aggregate_ranks.py

	echo "| Algorithm | Average Rank | Average Time (seconds) |" > aggregated_ranks.md
	echo "|---|---|---|" >> aggregated_ranks.md
	cat aggregated_ranks.csv | sed 's/.*/|&|/' | sed 's/\;/\|/g' >> aggregated_ranks.md
