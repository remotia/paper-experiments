vmaf -r results/videos/captured.y4m -d results/videos/rendered.y4m -o results/vmaf.csv --csv \
    --threads 8 \
    --feature psnr_hvs \
    --feature float_ssim

    # --feature psnr \
    # --feature float_ms_ssim
