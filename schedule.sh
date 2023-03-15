# for setup in "setup_1" "setup_2" "setup_3"
for setup in "setup_2" "setup_3"
do
    python3 scripts/key_measures/transmission.py \
        archive/games/dota2/$setup/aggregated/ \
        archive/games/borderlands2/$setup/aggregated/ \
        archive/games/companyofheroes2/$setup/aggregated/ \
        archive/games/dirt5/$setup/aggregated/ \
        archive/all/$setup/measures/transmission
done


# cd scripts/correlation_studies/

# python3 processing_times.py \
#     ../../archive/games/dota2/setup_1/aggregated/ \
#     ../../archive/games/dirt5/setup_1/aggregated/ \
#     ../../archive/games/companyofheroes2/setup_1/aggregated/ \
#     ../../archive/games/borderlands2/setup_1/aggregated/ \
#     ../../plots/correlation/processing_times.pdf

# python3 processing_jitter.py \
#     ../../archive/games/dota2/setup_1/aggregated/ \
#     ../../archive/games/dirt5/setup_1/aggregated/ \
#     ../../archive/games/companyofheroes2/setup_1/aggregated/ \
#     ../../archive/games/borderlands2/setup_1/aggregated/ \
#     ../../plots/correlation/processing_jitter/all.pdf

# python3 gamewise_processing_jitter.py \
#     ../../archive/games/dota2/setup_1/aggregated/ \
#     ../../archive/games/borderlands2/setup_1/aggregated/ \
#     ../../archive/games/dirt5/setup_1/aggregated/ \
#     ../../archive/games/companyofheroes2/setup_1/aggregated/ \
#     ../../plots/correlation/gamewise_processing_jitter.pdf

# python3 idle_times.py \
#     ../../archive/games/ \
#     ../../plots/correlation/idle_times.pdf

# python3 transmission_times.py \
#     ../../archive/games/ \
#     ../../plots/correlation/transmission_times.pdf

# export NETEM_SETUP=setup_1
# for file in configurations/borderlands2/*
# do 
#     ./scripts/docker/srt_transmission/run_and_analytics.sh $file
# done
# mkdir -p archive/borderlands2/$NETEM_SETUP/
# mv aggregated archive/borderlands2/$NETEM_SETUP/aggregated
