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
    -define png:compression-filter=1 \
    -define png:compression-level=9 \
    -define png:compression-strategy=2 \
    "$filename" "$outputfolder/$frame_id.png"