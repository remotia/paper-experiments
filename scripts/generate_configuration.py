import os
import sys
import toml
import copy
import hashlib

# Loading
base_conf = toml.load(open(f"configurations/.base.toml", "r"))

for video in os.listdir("videos/borderlands2/"):
    for crf in [26, 32, 36, 45]:
        for latency in [20, 50]:
            for max_frame_delay in [70, 120, 180]:
                conf = copy.deepcopy(base_conf)

                conf["video_file_path"] = f"videos/borderlands2/{video}"
                conf["encoder_options"]["crf"] = str(crf)

                conf["transmission"]["latency"] = latency
                conf["transmission"]["max_frame_delay"] = max_frame_delay

                conf_name = f"{video}_{crf}_{latency}_{max_frame_delay}"
                # conf_name = hashlib.sha1(str(conf).encode("UTF-8")).hexdigest()[:8]
                toml.dump(conf, open(f"configurations/borderlands2/{conf_name}.toml", "w"))
