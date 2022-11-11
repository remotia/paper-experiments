TYPE=$1

for config in configurations/* 
do
    configname=$(basename $config)
    echo "Running $configname"
    ./scripts/experiment/$TYPE.sh $config
    ./scripts/experiment/archive.sh $configname
done
