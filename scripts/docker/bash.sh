DOCKER_ROOT=/home/paper-experiments/

docker run -it \
    --cap-add=NET_ADMIN \
    --mount type=bind,source="$(pwd)/configurations/",target=$DOCKER_ROOT/configurations \
    --mount type=bind,source="$(pwd)/scripts/",target=$DOCKER_ROOT/scripts \
    --mount type=bind,source="$(pwd)/videos/",target=$DOCKER_ROOT/videos \
    --mount type=bind,source="$(pwd)/docker_mounts/results/",target=$DOCKER_ROOT/results \
    --network experiments \
    --entrypoint bash \
    remotia:auto_encoding 
