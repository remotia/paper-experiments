CURRENT_DIRECTORY=$(pwd)

DOCKER_ROOT=/home/paper-experiments/

docker run -it \
    --mount type=bind,source="$CURRENT_DIRECTORY/scripts/",target=$DOCKER_ROOT/scripts \
    --mount type=bind,source="$(pwd)/results/",target=$DOCKER_ROOT/results \
    --env WIDTH=$WIDTH \
    --env HEIGHT=$HEIGHT \
    --entrypoint bash \
    remotia:auto_encoding_analytics
