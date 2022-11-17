tc qdisc add dev lo root netem \
    delay 40ms \
    loss 0.2% 25% \
    rate 200mbit
