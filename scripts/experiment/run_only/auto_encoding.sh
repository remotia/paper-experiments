#!/bin/sh

EXECUTABLE=auto_encoding

rm -rf results/*

$EXECUTABLE --config-file-path $CONFIGURATION
# ./scripts/dump/video_pipeline.sh

# rm -rf results/dump/
# rm -rf results/compressed_dump/
