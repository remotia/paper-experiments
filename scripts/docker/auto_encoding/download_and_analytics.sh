if [[ ! -v RCLONE_REMOTE ]]; then
    echo "RCLONE_REMOTE is not set"
    exit
fi

CURRENT_DIRECTORY=$(pwd)
# rm -r analytics_workbench
# mkdir analytics_workbench

cd analytics_workbench

DOCKER_ROOT=/home/paper-experiments/

CONFIGURATION=$1

EXPERIMENT_ID=$(basename $CONFIGURATION)

rclone -P sync $RCLONE_REMOTE:/remotia-results-raw/$EXPERIMENT_ID/results.tar  .
tar -xf results.tar 
rm results.tar
mv docker_mounts/results/ results/

# TODO: Remove scripts mount

docker run -it \
    --mount type=bind,source="$CURRENT_DIRECTORY/scripts/",target=$DOCKER_ROOT/scripts \
    --mount type=bind,source="$(pwd)/results/",target=$DOCKER_ROOT/results \
    --name $EXPERIMENT_ID \
    --env WIDTH=$WIDTH \
    --env HEIGHT=$HEIGHT \
    --entrypoint ./scripts/analytics/auto_encoding/video_pipeline.sh \
    remotia:auto_encoding_analytics

    # --entrypoint bash \
docker rm $EXPERIMENT_ID

cd $CURRENT_DIRECTORY
mkdir -p aggregated/$EXPERIMENT_ID
cp -r analytics_workbench/results/stats aggregated/$EXPERIMENT_ID/stats/
cp analytics_workbench/results/vmaf.csv aggregated/$EXPERIMENT_ID/vmaf.csv
