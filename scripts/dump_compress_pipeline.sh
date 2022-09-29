rm -r compressed_dump/*

./scripts/compress_all.sh dump/input_frames/ compressed_dump/input_frames/ 1280 720
./scripts/compress_all.sh dump/decoded_frames/ compressed_dump/decoded_frames/ 1280 720
