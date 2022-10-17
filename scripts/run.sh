EXAMPLE_ID=$1
CONFIG_PATH=$2

export RUSTFLAGS="-C target-cpu=native"

cargo run --release --example $EXAMPLE_ID -- --config-file-path $CONFIG_PATH
