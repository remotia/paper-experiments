tc qdisc add dev lo root netem \
    loss 0.5% 25% \
    delay 120ms 40ms \
    rate 20mbit
