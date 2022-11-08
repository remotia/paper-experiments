#!/bin/sh
CAPTURED_FOLDER=$1
RENDERED_FOLDER=$2

for file in $CAPTURED_FOLDER/*
do
    test -f "$file" || continue
    filename=$(basename "$file")

    if [ ! -f "$RENDERED_FOLDER/$filename" ];
    then
        OUTFILE="$RENDERED_FOLDER/$filename"
        WIDTH=$(identify -format '%w' $file)
        HEIGHT=$(identify -format '%h' $file)
        echo "Compensating (${WIDTH}x${HEIGHT})..."
        convert -size ${WIDTH}x${HEIGHT} xc:black "$OUTFILE"
    fi

    exit 0 
done