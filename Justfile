set dotenv-load

run_all:
	@just run uniform
	@just run half_range
	@just run hard
	@just run beta
	@just run reverse_beta
	mdsh

run EXPERIMENT:
	rm -f {{EXPERIMENT}}.csv

	bkt -- cargo run --release -- {{EXPERIMENT}} bds >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} bge >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ebtci >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} epsts >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} fe >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} giro >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} gradient >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} greedy >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} klms >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} klucb >> {{EXPERIMENT}}.csv
	bkt --modtime=./src/bandits/lilucb.rs -- cargo run --release -- {{EXPERIMENT}} lilucb >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} mbe >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} npts >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ots >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} phe >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} reboot >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} random >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} sts >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ts >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} tsucb >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} tsvha >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} tsallisinf >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ucb1 >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ucbdt >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} ucb1tuned >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} vresboot >> {{EXPERIMENT}}.csv
	bkt -- cargo run --release -- {{EXPERIMENT}} wrsda >> {{EXPERIMENT}}.csv

	sort -n -t ';' -k 3,3 {{EXPERIMENT}}.csv -o {{EXPERIMENT}}.csv

	echo "| Algorithm | %-Optimal | Regret (Mean) | Regret (Median Absolute Deviation) | Time |" > {{EXPERIMENT}}.md
	echo "|---|---:|---:|---:|:--:|" >> {{EXPERIMENT}}.md

	cat {{EXPERIMENT}}.csv | sed 's/.*/|&|/' | sed 's/\;/\|/g' >> {{EXPERIMENT}}.md
