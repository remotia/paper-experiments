import shutil
import statistics
import toml
import os
import sys
import pandas

root_folder = sys.argv[1]

stats = list()

for folder_path in os.listdir(root_folder):
    try:
        row = list()

        config = toml.load(open(f"{root_folder}/{folder_path}/{folder_path}"))
        row.append(config['video_file_path'])
        row.append(config['encoder_options']['crf'])

        codec_stats = pandas.read_csv(f"{root_folder}/{folder_path}/results/stats/codec.csv")
        row.append(codec_stats['encoded_size'].mean())

        delay_stats = pandas.read_csv(f"{root_folder}/{folder_path}/results/stats/delay.csv")
        row.append(delay_stats['frame_delay'].mean())
        row.append(delay_stats['drop_reason'].notnull().sum() / delay_stats['capture_timestamp'].count())

        vmaf_stats = pandas.read_csv(f"{root_folder}/{folder_path}/results/vmaf.csv")
        row.append(vmaf_stats['psnr_hvs'].mean())
        row.append(vmaf_stats['float_ssim'].mean())
        row.append(vmaf_stats['vmaf'].mean())

        stats.append(row)
    except Exception as e:
        print(f"Unable to read {folder_path}: {e}")

stats = pandas.DataFrame(stats, columns = [
    "video_path", 
    "crf", 
    "encoded_size", 
    "frame_delay", 
    "drop_rate",
    "psnr_hvs",
    "float_ssim",
    "vmaf"
])

stats = stats.groupby("crf").mean(numeric_only=True)

max_values = stats.max()
min_values = stats.min()

def calculate_min_score(row, stat_id, importance):
    value = min(min_values[stat_id] / row[stat_id], 1.0)
    row[f"{stat_id}_score"] = value * importance
    row.drop(stat_id, inplace=True)
    return value

def calculate_max_score(row, stat_id, importance):
    value = min(row[stat_id] / max_values[stat_id], 1.0)
    row[f"{stat_id}_score"] = value * importance
    row.drop(stat_id, inplace=True)
    return value

def calculate_metrics_scores(row):
    calculate_min_score(row, "encoded_size", 0.5)
    calculate_min_score(row, "frame_delay", 1.5)
    calculate_min_score(row, "drop_rate", 0.5)

    calculate_max_score(row, "psnr_hvs", 1.0)
    calculate_max_score(row, "float_ssim", 1.0)
    calculate_max_score(row, "vmaf", 1.5)

    return row

directory = sys.argv[2]
shutil.rmtree(directory, ignore_errors=True)
os.makedirs(directory)

stats = stats.round(2)
stats.to_csv(f"{directory}/stats.csv")

metrics_scores = stats.apply(calculate_metrics_scores, axis = 1)
metrics_scores.to_csv(f"{directory}/metrics_scores.csv")

summary = stats
summary["score"] = metrics_scores.mean(axis=1)
summary.to_csv(f"{directory}/summary.csv")
print(summary)
