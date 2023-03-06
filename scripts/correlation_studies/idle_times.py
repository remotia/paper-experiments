import glob

import matplotlib.pyplot as plt
import numpy
import os
import sys
import pandas

import common

pandas.set_option('display.float_format', lambda x: '%.3f' % x)

root_folder = sys.argv[1]

print("Listing files...")
files = list(glob.glob(f"{root_folder}/**/idle.csv", recursive=True))

data = list()
for (i, file_path) in enumerate(files):
    print(f"Processing file {i+1}/{len(files)} ({file_path})...")

    # Load CSV file
    processing_data = pandas.read_csv(file_path)

    # Remove unnecessary columns:
    processing_data.drop(columns=["capture_timestamp"], index=1, inplace=True)

    experiment_data = processing_data.mean().round(2).to_frame().transpose()

    data.append(experiment_data)

df = pandas.concat(data).mean().to_frame()

for index in df.index:
    df.rename(index = {
        index: common.metric_id_to_display_name(index)
    }, inplace=True)

ax = df.plot.bar()
ax.get_legend().remove()
common.wrap_labels(ax, 10)
plt.xticks(rotation=0)
plt.xlabel("Component")
plt.ylabel("Idle time (ms)")
plt.savefig(sys.argv[-1], bbox_inches='tight',dpi=100)
