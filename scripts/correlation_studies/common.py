import textwrap

# From https://medium.com/dunder-data/automatically-wrap-graph-labels-in-matplotlib-and-seaborn-a48740bc9ce
def wrap_labels(ax, width, break_long_words=False):
    labels = []
    for label in ax.get_xticklabels():
        text = label.get_text()
        labels.append(textwrap.fill(text, width=width,
                      break_long_words=break_long_words))
    ax.set_xticklabels(labels, rotation=0)

def map_to_display_name(id, map):
    for key in map:
        if key in id:
            return map[key]

    return id

def path_to_game_name(folder_path):
    return map_to_display_name(folder_path, {
        "dota2": "DotA 2",
        "borderlands2": "Borderlands 2",
        "companyofheroes2": "Company of heroes 2",
        "dirt5": "Dirt 5"
    })

def metric_id_to_display_name(metric_id):
    return map_to_display_name(metric_id, {
        "capture": "Capture",
        "yuv420p_conversion": "YUV420P conversion",
        "encode_processing": "Encoding",
        "decode_processing": "Decoding",
        "rgba_conversion": "RGBA conversion",
    })
