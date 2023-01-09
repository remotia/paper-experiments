#!/bin/sh

EXECUTABLE=srt_transmission

rm -rf results/*

./netem/$NETEM_SETUP.sh
$EXECUTABLE --config-file-path $CONFIGURATION
./netem/reset.sh
