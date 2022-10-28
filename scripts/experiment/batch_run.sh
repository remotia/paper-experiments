for config in configurations/* 
do
    configname=$(basename $config)
    echo "Running $configname"
    ./scripts/experiment/run.sh target/release/examples/auto_encoding $config
    ./scripts/experiment/archive.sh $configname
done
