# ./scripts/dump/compensate_start_delay.sh results/dump/captured/ results/dump/rendered/

./scripts/dump/tag_timestamp.sh  results/dump/captured/
# ./scripts/dump/tag_timestamp.sh  results/dump/rendered/

rm -rf results/videos/
mkdir -p results/videos/
./scripts/dump/generate_video.sh  results/dump/captured/ results/videos/captured.y4m bgra
# rm -rf results/dump/captured
# ./scripts/dump/generate_video.sh  results/dump/rendered/ results/videos/rendered.y4m rgba
# rm -rf results/dump/rendered
