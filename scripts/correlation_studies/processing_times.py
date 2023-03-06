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
    game_data = list()
    for experiment_folder in os.listdir(game_folder):
        # Load CSV file
        processing_data = pandas.read_csv(f"{game_folder}/{experiment_folder}/stats/processing.csv")

        # Remove unnecessary columns:
        # capture_timestamp: identifies each frame, it is useless in this case
        # send_processing_time: legacy metrics, currently uncollected, to be removed

        processing_data.drop(columns=["capture_timestamp", "send_processing_time"], index=1, inplace=True)

        experiment_data = processing_data.mean().round(2).to_frame().transpose()
        experiment_data["experiment_id"] = experiment_folder
        experiment_data.set_index("experiment_id", inplace=True)

        game_data.append(experiment_data)

    folder_results = pandas.concat(game_data)

    mean = folder_results.mean().to_frame().transpose()
    mean["game_id"] = common.path_to_game_name(game_folder)
    mean.set_index("game_id", inplace=True)
    data.append(mean)

aggregated_means = pandas.concat(data)
for column in aggregated_means.columns:
    aggregated_means.rename(columns = {
        column: common.metric_id_to_display_name(column)
    }, inplace=True)

ax = aggregated_means.plot.bar(stacked=True)
ax.legend(ncol=1, loc=[1.05, 0.6])
common.wrap_labels(ax, 10)
plt.xticks(rotation=0)
plt.xlabel("Game")
plt.ylabel("Processing time (ms)")
plt.savefig(sys.argv[-1], bbox_inches='tight',dpi=100)
