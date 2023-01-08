#!/bin/sh

EXECUTABLE=srt_transmission

rm -rf results/*

$EXECUTABLE --config-file-path $CONFIGURATION
