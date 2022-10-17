EXAMPLE_ID=$1
CONFIG_PATH=$2

cargo run --release --example $EXAMPLE_ID -- --config-file-path $CONFIG_PATH