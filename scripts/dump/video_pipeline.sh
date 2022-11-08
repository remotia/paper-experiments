rm -rf results/compressed_dump/
mkdir -p results/compressed_dump/
 
./scripts/compress_all.sh results/dump/captured/ results/compressed_dump/captured/ 1280 720
./scripts/compress_all.sh results/dump/rendered/ results/compressed_dump/rendered/ 1280 720

./scripts/dump/compensate_start_delay.sh results/compressed_dump/captured/ results/compressed_dump/rendered/

./scripts/dump/tag_timestamp.sh  results/compressed_dump/captured/
./scripts/dump/tag_timestamp.sh  results/compressed_dump/rendered/

rm -rf results/videos/
mkdir -p results/videos/
./scripts/dump/generate_video.sh  results/compressed_dump/captured/ results/videos/captured.y4m
rm -rf results/compressed_dump/captured
./scripts/dump/generate_video.sh  results/compressed_dump/rendered/ results/videos/rendered.y4m
rm -rf results/compressed_dump/rendered
