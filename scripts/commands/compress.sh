#!/bin/bash
filename=$1
outputfolder=$2
width=$3
height=$4

mkdir -p $outputfolder

base=$(basename $filename)
frame_id="${base%.*}"
echo "Converting $filename..."
convert -size ${width}x${height} -depth 8 \
    -colorspace RGB \
    -define webp:lossless=true \
    -define webp:method=0 \
    -define webp:threaded-level=1 \
    "$filename" "$outputfolder/$frame_id.webp"