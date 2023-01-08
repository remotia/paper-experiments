DOCKER_ROOT=/home/paper-experiments/

CONFIGURATION=$1

# TODO: Add type parameter again
# TODO: Remove scripts mount
CONFIGURATION_ID="$(basename $CONFIGURATION)"
EXPERIMENT_ID="${CONFIGURATION_ID}_$(date +%s)"

docker rm "$EXPERIMENT_ID"
docker run -it \
    --mount type=bind,source="$(pwd)/docker_mounts/results/",target=$DOCKER_ROOT/results/\
    --mount type=bind,source="$(pwd)/analytics_workbench/docker_mounts/results/",target=$DOCKER_ROOT/analytics_results/ \
    --name "$EXPERIMENT_ID" \
    --entrypoint rm \
    remotia:auto_encoding_analytics -rf results/* analytics_results/*

docker rm "$EXPERIMENT_ID"
docker run -it \
    --cap-add=NET_ADMIN \
    --mount type=bind,source="$(pwd)/configurations/",target=$DOCKER_ROOT/configurations \
    --mount type=bind,source="$(pwd)/scripts/",target=$DOCKER_ROOT/scripts \
    --mount type=bind,source="$(pwd)/videos/",target=$DOCKER_ROOT/videos \
    --mount type=bind,source="$(pwd)/docker_mounts/results/",target=$DOCKER_ROOT/results \
    --network experiments \
    --name "$EXPERIMENT_ID" \
    --env CONFIGURATION=$CONFIGURATION \
    --entrypoint ./scripts/experiment/run_only/srt_transmission.sh \
    remotia:srt_transmission

# Analytics
CURRENT_DIRECTORY=$(pwd)

DOCKER_ROOT=/home/paper-experiments/

# TODO: Remove scripts mount

docker rm "$EXPERIMENT_ID"
docker run -it \
    --mount type=bind,source="$CURRENT_DIRECTORY/scripts/",target=$DOCKER_ROOT/scripts \
    --mount type=bind,source="$CURRENT_DIRECTORY/docker_mounts/results/",target=$DOCKER_ROOT/results \
    --name "$EXPERIMENT_ID" \
    --env WIDTH=$WIDTH \
    --env HEIGHT=$HEIGHT \
    --entrypoint ./scripts/analytics/auto_encoding/video_pipeline.sh \
    remotia:auto_encoding_analytics

    # --entrypoint bash \

cd $CURRENT_DIRECTORY
mkdir -p aggregated/"$CONFIGURATION_ID"
cp -r docker_mounts/results/stats aggregated/"$CONFIGURATION_ID"/
cp docker_mounts/results/vmaf.csv aggregated/"$CONFIGURATION_ID"/vmaf.csv
cp $CONFIGURATION aggregated/"$CONFIGURATION_ID"/
