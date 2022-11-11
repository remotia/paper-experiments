import toml
import os
import sys
import pandas

root_folder = sys.argv[1]

summary = list()

for folder_path in os.listdir(root_folder):
    try:
        row = list()

        config = toml.load(open(f"{root_folder}/{folder_path}/{folder_path}"))
        row.append(config['video_file_path'])
        row.append(config['transmission']['latency'])
        row.append(config['encoder_options']['crf'])
        keyint = config['encoder_options']['x264opts'].replace("keyint=", "")
        row.append(keyint)

        codec_stats = pandas.read_csv(f"{root_folder}/{folder_path}/results/stats/codec.csv")
        row.append(codec_stats['encoded_size'].mean())

        delay_stats = pandas.read_csv(f"{root_folder}/{folder_path}/results/stats/delay.csv")
        row.append(delay_stats['frame_delay'].mean())
        row.append(delay_stats['drop_reason'].notnull().sum() / delay_stats['capture_timestamp'].count())

        vmaf_stats = pandas.read_csv(f"{root_folder}/{folder_path}/results/vmaf.csv")
        row.append(vmaf_stats['psnr_hvs'].mean())
        row.append(vmaf_stats['float_ssim'].mean())
        row.append(vmaf_stats['vmaf'].mean())

        summary.append(row)
    except Exception as e:
        print(f"Unable to read {folder_path}: {e}")

df_summary = pandas.DataFrame(summary, columns = [
    "video_path", 
    "latency",
    "crf", 
    "keyint",
    "encoded_size", 
    "frame_delay", 
    "drop_rate",
    "psnr_hvs",
    "float_ssim",
    "vmaf"
])

grouped_summary = df_summary.groupby(["latency", "crf", "keyint"]).mean()
grouped_summary.to_csv(sys.argv[2])
print(grouped_summary)