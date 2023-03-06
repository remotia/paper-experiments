import matplotlib.pyplot as plt
import numpy
import os
import sys
import pandas

import common

pandas.set_option('display.float_format', lambda x: '%.3f' % x)

game_folders = sys.argv[1:-1]

mean_data = list()
std_data = list()
for game_folder in game_folders:
    print(f"Processing {game_folder}...")
    game_mean_data = list()
    game_std_data = list()
    for experiment_folder in os.listdir(game_folder):
        # Load CSV file
        processing_data = pandas.read_csv(f"{game_folder}/{experiment_folder}/stats/processing.csv")

        # Remove unnecessary columns:
        # capture_timestamp: identifies each frame, it is useless in this case
        # send_processing_time: legacy metrics, currently uncollected, to be removed
        processing_data.drop(columns=["capture_timestamp", "send_processing_time"], index=1, inplace=True)

        # Calculate mean and standard deviation
        mean = processing_data.mean().to_frame().transpose()
        std = processing_data.std().to_frame().transpose()

        game_mean_data.append(mean)
        game_std_data.append(std)

    # Average the aggregated data
    aggregated_game_means = pandas.concat(game_mean_data).mean().to_frame().transpose()
    aggregated_game_means["game_id"] = common.path_to_game_name(game_folder)
    aggregated_game_means.set_index("game_id", inplace=True)

    aggregated_game_std = pandas.concat(game_std_data).mean().to_frame().transpose() 
    aggregated_game_std["game_id"] = common.path_to_game_name(game_folder)
    aggregated_game_std.set_index("game_id", inplace=True)

    mean_data.append(aggregated_game_means)
    std_data.append(aggregated_game_std)

aggregated_mean_data = pandas.concat(mean_data).transpose()
aggregated_std_data = pandas.concat(std_data).transpose()

aggregated_std_percentage_data = aggregated_std_data.divide(aggregated_mean_data) * 100.0

# Plot the results
for index in aggregated_std_percentage_data.index:
    aggregated_std_percentage_data.rename(index = {
        index: common.metric_id_to_display_name(index)
    }, inplace=True)

ax = aggregated_std_percentage_data.plot.bar(
    color=["blue", "orange", "red", "green"]
)

common.wrap_labels(ax, 10)
ax.legend(ncol=1, loc=[1.05, 0.6])
plt.xticks(rotation=0)
plt.xlabel("Measure")
plt.ylabel("Processing time jitter (%)")
plt.savefig(sys.argv[-1], bbox_inches='tight',dpi=100)

