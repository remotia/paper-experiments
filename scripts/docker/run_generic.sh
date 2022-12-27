docker run -it \
    --entrypoint /bin/bash \
    --cap-add=NET_ADMIN \
    --mount type=bind,source="$(pwd)/videos/",target=/home/remotia-experiments/videos \
    --name $1 \
    --network experiments \
    remotia-experiments
