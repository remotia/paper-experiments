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
delay_files = list(glob.glob(f"{root_folder}/**/delay.csv", recursive=True))

delay_data = list()
for (i, file_path) in enumerate(delay_files):
    print(f"Processing file {i+1}/{len(delay_files)} ({file_path})...")
    delay_csv_data = pandas.read_csv(file_path)
    delay_csv_data.drop(columns=["capture_timestamp", "send_delay", "frame_delay", "drop_reason"], index=1, inplace=True)
    delay_experiment_data = delay_csv_data.mean().round(2).to_frame().transpose()
    delay_data.append(delay_experiment_data)

processing_files = list(glob.glob(f"{root_folder}/**/processing.csv", recursive=True))

processing_data = list()
for (i, file_path) in enumerate(processing_files):
    print(f"Processing file {i+1}/{len(processing_files)} ({file_path})...")
    processing_csv_data = pandas.read_csv(file_path)
    processing_csv_data.drop(
        columns=["capture_timestamp", "capture_processing_time", "send_processing_time"], 
        index=1, 
        inplace=True
    )
    processing_experiment_data = processing_csv_data.mean().round(2).to_frame().transpose()
    processing_data.append(processing_experiment_data)

delay_df = pandas.concat(delay_data).mean().to_frame()
processing_df = pandas.concat(processing_data).mean().to_frame()

delay_df.index = delay_df.index.str.replace("_delay", "")
processing_df.index = processing_df.index.str.replace("_processing_time", "")

delay_df = delay_df.transpose()
delay_df["type"] = "delay"
delay_df.set_index("type", inplace=True)

processing_df = processing_df.transpose()
processing_df["type"] = "processing"
processing_df.set_index("type", inplace=True)

df = pandas.concat((delay_df, processing_df))

print(df)

sys.exit(0)

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
