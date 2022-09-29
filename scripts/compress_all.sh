#!/bin/bash
# find $1/* | parallel --progress ./commands/compress.sh "{}" $2
for file in $1/*
do
    ./scripts/commands/compress.sh $file $2 $3 $4
done