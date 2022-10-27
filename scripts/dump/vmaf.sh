vmaf -r results/videos/captured.y4m -d results/videos/rendered.y4m -o results/vmaf.csv --csv \
    --feature psnr \
    --feature psnr_hvs \
    --feature float_ssim \
    --feature float_ms_ssim
