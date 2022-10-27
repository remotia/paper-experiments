FOLDER=$1
OUTPUT=$2
ffmpeg -f image2 -ts_from_file 2 -pattern_type glob -i "$1/*.png" -c:v libx264 -qp 0 "$2"
