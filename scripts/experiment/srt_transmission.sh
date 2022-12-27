EXECUTABLE=bin/srt_transmission
CONFIGURATION=$1

rm -rf results/

$EXECUTABLE --config-file-path $CONFIGURATION
rm -rf results/dump/
