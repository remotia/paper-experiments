tc qdisc add dev lo root netem \
    delay 20ms \
    loss 0.1% 25% \
    rate 1000mbit
