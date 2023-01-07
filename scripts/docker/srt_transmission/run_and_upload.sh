DOCKER_ROOT=/home/paper-experiments/

if [[ ! -v RCLONE_REMOTE ]]; then
    echo "RCLONE_REMOTE is not set"
    exit
fi

CONFIGURATION=$1

# TODO: Add type parameter again
# TODO: Remove scripts mount
EXPERIMENT_ID=$(basename $CONFIGURATION)

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

tar -cf results.tar docker_mounts/results/
rclone -P sync results.tar $RCLONE_REMOTE:/remotia-results-raw/$EXPERIMENT_ID/results.tar
rm results.tar

docker rm $EXPERIMENT_ID
