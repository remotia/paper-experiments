tc qdisc add dev lo root netem \
    delay 80ms \
    loss 0.5% 25% \
    rate 100mbit
