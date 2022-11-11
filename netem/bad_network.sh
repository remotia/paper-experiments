tc qdisc add dev lo root netem \
    loss 5% \
    delay 150ms \
    rate 20mbit
