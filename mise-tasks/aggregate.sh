#!/bin/bash
#MISE description="Aggregate rankings from all experiments"
#MISE wait_for=["experiments"]

uv run aggregate_ranks.py

echo "| Algorithm | Average Rank | Average Time (seconds) |" > aggregated_ranks.csv
echo "|---|---|---|" >> aggregated_ranks.csv
cat aggregated_ranks.csv | sed 's/.*/|&|/' | sed 's/\;/\|/g' >> aggregated_ranks.csv
