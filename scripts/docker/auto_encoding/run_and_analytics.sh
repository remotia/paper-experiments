DOCKER_ROOT=/home/paper-experiments/

CONFIGURATION=$1

# TODO: Add type parameter again
# TODO: Remove scripts mount
EXPERIMENT_ID=$(basename $CONFIGURATION)

docker rm results_remove
docker run -it \
    --mount type=bind,source="$(pwd)/docker_mounts/results/",target=$DOCKER_ROOT/results/\
    --mount type=bind,source="$(pwd)/analytics_workbench/docker_mounts/results/",target=$DOCKER_ROOT/analytics_results/ \
    --name results_remove \
    --entrypoint rm \
    remotia:auto_encoding_analytics -rf results/* analytics_results/*

docker run -it \
    --cap-add=NET_ADMIN \
    --mount type=bind,source="$(pwd)/configurations/",target=$DOCKER_ROOT/configurations \
    --mount type=bind,source="$(pwd)/scripts/",target=$DOCKER_ROOT/scripts \
    --mount type=bind,source="$(pwd)/videos/",target=$DOCKER_ROOT/videos \
    --mount type=bind,source="$(pwd)/docker_mounts/results/",target=$DOCKER_ROOT/results \
    --network experiments \
    --name $EXPERIMENT_ID \
    --env CONFIGURATION=$CONFIGURATION \
    --entrypoint ./scripts/experiment/run_only/auto_encoding.sh \
    remotia:auto_encoding 

docker rm $EXPERIMENT_ID

# Analytics
CURRENT_DIRECTORY=$(pwd)

DOCKER_ROOT=/home/paper-experiments/

# TODO: Remove scripts mount

docker run -it \
    --mount type=bind,source="$CURRENT_DIRECTORY/scripts/",target=$DOCKER_ROOT/scripts \
    --mount type=bind,source="$CURRENT_DIRECTORY/docker_mounts/results/",target=$DOCKER_ROOT/results \
    --name $EXPERIMENT_ID \
    --env WIDTH=$WIDTH \
    --env HEIGHT=$HEIGHT \
    --entrypoint ./scripts/analytics/auto_encoding/video_pipeline.sh \
    remotia:auto_encoding_analytics

    # --entrypoint bash \
docker rm $EXPERIMENT_ID

cd $CURRENT_DIRECTORY
mkdir -p aggregated/$EXPERIMENT_ID
cp -r docker_mounts/results/stats aggregated/$EXPERIMENT_ID/
cp docker_mounts/results/vmaf.csv aggregated/$EXPERIMENT_ID/vmaf.csv

