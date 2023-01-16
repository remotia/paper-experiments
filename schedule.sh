for game in "dirt5" "borderlands2"
do
    for setup in "setup_1" "setup_2" "setup_3"
    do
        echo "$game $setup"
        python3 scripts/key_measures/transmission.py \
            archive/$game/$setup/aggregated/ \
            archive/$game/$setup/measures/transmission
            exit
    done
done

# export NETEM_SETUP=setup_1
# for file in configurations/borderlands2/*
# do 
#     ./scripts/docker/srt_transmission/run_and_analytics.sh $file
# done
# mkdir -p archive/borderlands2/$NETEM_SETUP/
# mv aggregated archive/borderlands2/$NETEM_SETUP/aggregated
