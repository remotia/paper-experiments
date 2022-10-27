rm -rf results/compressed_dump/
mkdir -p results/compressed_dump/
 
./scripts/compress_all.sh results/dump/captured/ results/compressed_dump/captured/ 1280 720
./scripts/compress_all.sh results/dump/rendered/ results/compressed_dump/rendered/ 1280 720

./scripts/dump/tag_timestamp.sh  results/compressed_dump/captured/
./scripts/dump/tag_timestamp.sh  results/compressed_dump/rendered/

rm -rf results/videos/
mkdir -p results/videos/
./scripts/dump/video_pipeline.sh  results/compressed_dump/captured/ results/videos/captured.mkv
./scripts/dump/video_pipeline.sh  results/compressed_dump/rendered/ results/videos/rendered.mkv
