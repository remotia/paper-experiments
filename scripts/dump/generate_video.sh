FOLDER=$1
OUTPUT=$2
PIX_FMT=$3

CONTAINER=./results/videos/tmp.mkv

# For some random reason encoding directly to y4m is utterly difficult
ffmpeg -f image2 -ts_from_file 2 -pattern_type glob -vcodec rawvideo -s ${WIDTH}x${HEIGHT} -pix_fmt $PIX_FMT -i "$1/*.$PIX_FMT" -threads 4 -c:v libx264 -crf 18 -pix_fmt yuv420p -preset ultrafast $CONTAINER
# ffmpeg -i $CONTAINER -threads 4 -pix_fmt yuv420p "$2"
# rm $CONTAINER
