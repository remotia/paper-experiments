EXECUTABLE=target/release/examples/auto_encoding
CONFIGURATION=$1

rm -rf results/

$EXECUTABLE --config-file-path $CONFIGURATION
./scripts/dump/video_pipeline.sh
./scripts/dump/vmaf.sh

rm -rf results/dump/
rm -rf results/compressed_dump/
rm -rf results/videos/
