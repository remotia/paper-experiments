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
        row.append(folder_path)
        row.append(config['video_file_path'])
        row.append(config['encoder_options']['crf'])
        row.append(config['transmission']['latency'])
        row.append(config['transmission']['max_frame_delay'])

        delay_stats = pandas.read_csv(f"{root_folder}/{folder_path}/results/stats/delay.csv")
        drop_rate = float(delay_stats['drop_reason'].notnull().sum() / 900)
        if drop_rate > 0.25:
            continue


        rendered_frames_delay_stats = delay_stats.loc[delay_stats["drop_reason"].isna()]
        if rendered_frames_delay_stats.empty:
            continue

        codec_stats = pandas.read_csv(f"{root_folder}/{folder_path}/results/stats/codec.csv")
        joint_stats = rendered_frames_delay_stats.merge(codec_stats, on="capture_timestamp") 

        row.append(joint_stats['encoded_size'].mean())
        row.append(drop_rate)
        row.append(joint_stats['frame_delay'].mean())

        stats.append(row)
    except Exception as e:
        print(f"Unable to read {folder_path}: {e}")

stats = pandas.DataFrame(stats, columns = [
    "folder_path",
    "video_path", 
    "crf", 
    "latency",
    "max_frame_delay",
    "encoded_size",
    "drop_rate",
    "frame_delay", 
])

directory = sys.argv[2]
shutil.rmtree(directory, ignore_errors=True)
os.makedirs(directory)

stats.to_csv(f"{directory}/full.csv")

stats = stats.groupby(["crf", "latency", "max_frame_delay"]).mean(numeric_only=True)

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
    calculate_min_score(row, "frame_delay", 1.5)
    calculate_min_score(row, "drop_rate", 0.5)
    calculate_min_score(row, "encoded_size", 0.5)

    return row

stats = stats.round(2)
stats.to_csv(f"{directory}/stats.csv")

metrics_scores = stats.apply(calculate_metrics_scores, axis = 1)
metrics_scores.to_csv(f"{directory}/metrics_scores.csv")

summary = stats
summary["score"] = metrics_scores.mean(axis=1)
summary.sort_values("score", ascending=False, inplace=True)
summary.to_csv(f"{directory}/summary.csv")
print(summary)
