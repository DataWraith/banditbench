# /// script
# dependencies = [
#   "pandas"
# ]
# ///

import csv
import pandas as pd

def aggregate_rankings(csv_files, output_file):
    """
    Aggregate rankings from multiple CSV files.

    Args:
    csv_files (list): List of CSV file paths.
    output_file (str): Output CSV file path.
    """

    entry_ranks = {}
    entry_times = {}

    for file in csv_files:
        num_rows = 0

        with open(file, 'r') as f:
            reader = csv.reader(f, delimiter=';')
            for row in reader:
                num_rows += 1
                rank = num_rows

                entry = row[0]
                time = float(row[4][0:-1])

                if entry in entry_ranks:
                    entry_ranks[entry].append(rank)
                else:
                    entry_ranks[entry] = [rank]

                if entry in entry_times:
                    entry_times[entry].append(time)
                else:
                    entry_times[entry] = [time]


    # Calculate the average rank and time for each entry
    averaged_ranks = []
    for entry, ranks in entry_ranks.items():
        average_rank = sum(ranks) / len(ranks)
        averaged_ranks.append({'Algorithm': entry, 'Average Rank': average_rank, 'Average Time': round(sum(entry_times[entry]) / len(entry_times[entry]), 2)})

    averaged_ranks.sort(key=lambda x: x['Average Rank'])

    df = pd.DataFrame(averaged_ranks)
    df.to_csv(output_file, index=False, sep=';', header=False)

if __name__ == '__main__':
    csv_files = ['uniform.csv', 'half_range.csv', 'hard.csv', 'beta.csv', 'reverse_beta.csv']
    output_file = 'aggregated_ranks.csv'

    aggregate_rankings(csv_files, output_file)
