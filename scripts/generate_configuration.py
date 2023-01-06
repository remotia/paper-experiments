import os
import sys
import toml
import copy
import hashlib

# Loading
base_conf = toml.load(open(f"configurations/.base.toml", "r"))

for video in os.listdir("videos/dota2/")[8:]:
    for crf in [26, 32, 36]:
        conf = copy.deepcopy(base_conf)

        conf["video_file_path"] = f"videos/dota2/{video}"
        conf["encoder_options"]["crf"] = str(crf)

        conf_name = f"{video}_{crf}"
        # conf_name = hashlib.sha1(str(conf).encode("UTF-8")).hexdigest()[:8]
        toml.dump(conf, open(f"configurations/dota2/{conf_name}.toml", "w"))
