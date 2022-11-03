#!/bin/sh
echo "Tagging files in $1..."

for file in $1/*
do
    test -f "$file" || continue
    filename=$(basename "$file" .webp)
    full_ts="${filename%.*}"
    pre=$(echo $full_ts | cut -c1-10)
    milliseconds=$(echo $full_ts | cut -c11-13)
    datetime=$(date -d @$pre.$milliseconds +"%m/%d/%Y %H:%M:%S.%3N")
    touch --date "$datetime" $file
done