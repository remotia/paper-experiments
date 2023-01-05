FOLDER=$1
OUTPUT=$2
PIX_FMT=$3

CONTAINER=./results/videos/tmp.avi

# For some random reason encoding directly to y4m is utterly difficult
ffmpeg -y \
    -f image2 -ts_from_file 2 -pattern_type glob -vcodec rawvideo -s ${WIDTH}x${HEIGHT} \
    -pix_fmt $PIX_FMT -i "$1/*.$PIX_FMT" \
    -threads 4 -c:v rawvideo $CONTAINER

ffmpeg -y -i $CONTAINER -threads 4 -pix_fmt yuv420p -r 70 "$2"
rm $CONTAINER
