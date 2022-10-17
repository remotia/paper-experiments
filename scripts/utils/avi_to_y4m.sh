#!/bin/bash
shopt -s globstar

FOLDER="$1"
OUTPUT_FOLDER="$2"

for file_path in $FOLDER/**
do
    basename="$(basename "$file_path")"
    video_id="${basename%.*}"
    extension="${basename##*.}"

    if [[ $extension == "avi" ]]
    then
        ffmpeg -i $file_path -vf scale="1280:720" -r 30 "$OUTPUT_FOLDER/$video_id.y4m"
    fi 
done