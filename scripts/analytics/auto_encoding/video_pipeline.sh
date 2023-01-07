#!/bin/bash
ROOT=./scripts/analytics/auto_encoding

$ROOT/compensate_start_delay.sh results/dump/captured/ results/dump/rendered/

$ROOT/tag_timestamp.sh  results/dump/captured/
$ROOT/tag_timestamp.sh  results/dump/rendered/

rm -rf results/videos/
mkdir -p results/videos/
$ROOT/generate_video.sh  results/dump/captured/ results/videos/captured.y4m bgra
rm -rf results/dump/captured
$ROOT/generate_video.sh  results/dump/rendered/ results/videos/rendered.y4m rgba
rm -rf results/dump/rendered

$ROOT/vmaf.sh
