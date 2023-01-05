#!/bin/sh
CAPTURED_FOLDER=$1
RENDERED_FOLDER=$2

for file in $CAPTURED_FOLDER/*
do
    test -f "$file" || continue
    filename=$(basename "$file")

    if [ ! -f "$RENDERED_FOLDER/$filename" ];
    then
        file_id="${filename%.*}"
        OUTFILE="$RENDERED_FOLDER/$file_id.rgba"
        echo "Compensating $OUTFILE..."
        convert -size ${WIDTH}x${HEIGHT} xc:black "$OUTFILE"
    fi

    exit 0 
done