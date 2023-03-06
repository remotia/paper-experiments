# Currently unused in the paper

import matplotlib.pyplot as plt
import numpy
import os
import sys
import pandas

import common

pandas.set_option('display.float_format', lambda x: '%.3f' % x)

game_folders = sys.argv[1:-1]

data = list()
for game_folder in game_folders:
    print(f"Processing {game_folder}...")
    for experiment_folder in os.listdir(game_folder):
        # Load CSV file
        processing_data = pandas.read_csv(f"{game_folder}/{experiment_folder}/stats/processing.csv")

        # Remove unnecessary columns:
        # capture_timestamp: identifies each frame, it is useless in this case
        # send_processing_time: legacy metrics, currently uncollected, to be removed
        processing_data.drop(columns=["capture_timestamp", "send_processing_time"], index=1, inplace=True)

        # Calculate mean and standard deviation
        mean = processing_data.mean()
        std = processing_data.std()

        experiment_data = pandas.DataFrame({"mean": mean, "std": std})

        data.append(experiment_data)

# Average the aggregated data
aggregated_means = pandas.concat(data).groupby(level=0).mean().reindex([
    "capture_processing_time",
    "yuv420p_conversion_processing_time",
    "encode_processing_time",
    "decode_processing_time",
    "rgba_conversion_processing_time",
])

# Plot the results
for index in aggregated_means.index:
    aggregated_means.rename(index = {
        index: common.metric_id_to_display_name(index)
    }, inplace=True)

ax = aggregated_means["mean"].plot.bar(
    yerr=aggregated_means["std"], 
    capsize=5, 
    color=["orange"]
)

common.wrap_labels(ax, 10)
plt.xticks(rotation=0)
plt.xlabel("Measure")
plt.ylabel("Processing time (ms)")
plt.savefig(sys.argv[-1], bbox_inches='tight',dpi=100)

