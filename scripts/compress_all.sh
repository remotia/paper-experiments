#!/bin/bash
# find $1/* | parallel --progress ./commands/compress.sh "{}" $2
:> tasks.txt
for file in $1/*
do
    echo "./scripts/commands/compress.sh $file $2 $3 $4" >> tasks.txt
done

parallel < tasks.txt
rm tasks.txt