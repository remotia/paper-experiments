DOCKER_ROOT=/home/remotia-experiments/

if [[ ! -v RCLONE_REMOTE ]]; then
    echo "RCLONE_REMOTE is not set"
    exit
fi

TYPE=$1
CONFIGURATION=$2

docker run -it \
    --cap-add=NET_ADMIN \
    --mount type=bind,source="$(pwd)/bin/",target=$DOCKER_ROOT/bin \
    --mount type=bind,source="$(pwd)/videos/",target=$DOCKER_ROOT/videos \
    --mount type=bind,source="$(pwd)/configurations/",target=$DOCKER_ROOT/configurations \
    --mount type=bind,source="$(pwd)/scripts/",target=$DOCKER_ROOT/scripts \
    --mount type=bind,source="$(pwd)/docker_mounts/results/",target=$DOCKER_ROOT/results \
    --network experiments \
    --name experiment_container \
    --workdir "$DOCKER_ROOT" \
    --env CONFIGURATION=$CONFIGURATION \
    --env WIDTH=$WIDTH \
    --env HEIGHT=$HEIGHT \
    --entrypoint ./scripts/experiment/run_only/auto_encoding.sh \
    remotia-experiments

# rclone -P sync docker_mounts/results/ $RCLONE_REMOTE:/remotia-results/$CONFIGURATION
docker rm experiment_container
