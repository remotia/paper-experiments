FOLDER=$1
OUTPUT=$2

# For some random reason encoding directly to y4m is utterly difficult
ffmpeg -f image2 -ts_from_file 2 -pattern_type glob -i "$1/*.webp" -c:v libx264 -qp 0 tmp.mkv
ffmpeg -i tmp.mkv -pix_fmt yuv420p "$2"
rm tmp.mkv
