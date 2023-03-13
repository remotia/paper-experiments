import glob

import matplotlib.pyplot as plt
import numpy
import os
import sys
import pandas

import common

pandas.set_option('display.float_format', lambda x: '%.3f' % x)

root_folder = sys.argv[1]

def average(csv_data):
    csv_data = csv_data.drop(columns=["capture_timestamp", "drop_reason"])
    return csv_data.mean().round(2).to_frame().transpose()

def aggregate(data, type):
    df = pandas.concat(data).mean().to_frame()
    df.index = df.index.str.replace("_time", "")
    df = df.transpose()
    df["type"] = type
    df.set_index("type", inplace=True)
    return df

transmission_files = list(glob.glob(f"{root_folder}/**/transmission_delay.csv", recursive=True))
transmission_data = list()
no_error_transmission_data = list()
error_transmission_data = list()
for (i, file_path) in enumerate(transmission_files):
    print(f"transmission file {i+1}/{len(transmission_files)} ({file_path})...")
    transmission_csv_data = pandas.read_csv(file_path)
    no_error_transmission_csv_data = transmission_csv_data[~transmission_csv_data["drop_reason"].notnull()]
    error_transmission_csv_data = transmission_csv_data[transmission_csv_data["drop_reason"].notnull()]

    transmission_data.append(average(transmission_csv_data))
    no_error_transmission_data.append(average(no_error_transmission_csv_data))
    error_transmission_data.append(average(error_transmission_csv_data))

transmission_df = aggregate(transmission_data, "All")
no_error_transmission_df = aggregate(no_error_transmission_data, "Delivered frames")
error_transmission_df = aggregate(error_transmission_data, "Dropped frames")

df = pandas.concat((transmission_df, error_transmission_df, no_error_transmission_df)).transpose()

print(df)

for index in df.index:
    df.rename(index = {
        index: common.metric_id_to_display_name(index)
    }, inplace=True)

df = df.rename_axis(None, axis=1)

ax = df.plot.bar(color=["blue", "red", "green"])
common.wrap_labels(ax, 10)
plt.xticks(rotation=0)
plt.xlabel("Component")
plt.ylabel("Transmission time (ms)")
plt.savefig(sys.argv[-1], bbox_inches='tight',dpi=100)
