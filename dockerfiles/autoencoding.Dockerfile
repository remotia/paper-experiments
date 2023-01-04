FROM rust:1.66-buster
WORKDIR /home/paper-experiments/
COPY src/ .
COPY scripts/ .
COPY Cargo.toml/ .

ENTRYPOINT [ "/bin/bash" ]
